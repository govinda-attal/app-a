use super::model::{Auction, AuctionStatus, AuctionStatuses, Bid, BidStatus};
use crate::prelude::*;
use sqlx::postgres::PgHasArrayType;
use sqlx::{postgres::PgRow, PgPool, Postgres, Transaction};
use sqlx::{Execute, Executor, QueryBuilder, Row};
use uuid::Uuid;

pub async fn create_auction(tx: &mut Transaction<'_, Postgres>, a: Auction) -> Result<Auction> {
    let row = sqlx::query_as::<_, Auction>(
        r#"INSERT INTO AUCTIONS
            (ITEM, DESCRIPTION, SELLER, START_PRICE, CURRENT_PRICE, STATUS)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *"#,
    )
    .bind(&a.item)
    .bind(&a.description)
    .bind(&a.seller)
    .bind(a.start_price)
    .bind(a.current_price)
    .bind(&a.status)
    .fetch_one(&mut *tx)
    .await?;
    Ok(row)
}

pub async fn update_auction_status(
    tx: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    status: AuctionStatus,
) -> Result<Auction> {
    let ex_statuses = match status {
        AuctionStatus::Open => vec![AuctionStatus::Draft],
        AuctionStatus::Closed => vec![AuctionStatus::Open],
        AuctionStatus::Cancelled => vec![AuctionStatus::Draft, AuctionStatus::Open],
        _ => Err(Error::BadRquest(
            "invalid auction status transition".to_string(),
        ))?,
    };

    let mut qb = QueryBuilder::new("UPDATE AUCTIONS SET STATUS = ");

    qb.push_bind(&status)
        .push(" WHERE ID = ")
        .push_bind(id)
        .push(" AND STATUS in (");

    let mut ex_status_sep = qb.separated(", ");
    for v in ex_statuses.iter() {
        ex_status_sep.push_bind(v);
    }
    ex_status_sep.push_unseparated(") RETURNING *;");

    let rs = qb.build_query_as::<Auction>().fetch_one(&mut *tx).await;
    let Ok(auction) = rs else {
        return rs.map_err(|e| -> Error{
            match e {
                sqlx::Error::RowNotFound =>Error::BadRquest(f!("{} auction either not found or invalid status update", id)),
                _ => Error::from(e)
            }
        });
    };
    Ok(auction)
}

pub async fn overturn_bid(tx: &mut Transaction<'_, Postgres>, id: &Uuid) -> Result<Bid> {
    let row = sqlx::query_as::<_, Bid>(r#"UPDATE BIDS SET STATUS = $1 WHERE ID = $2 RETURNING *"#)
        .bind(&BidStatus::OverTurned)
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;
    Ok(row)
}

pub async fn fetch_auction(
    con: impl Executor<'_, Database = Postgres>,
    auction_id: &Uuid,
) -> Result<Option<Auction>> {
    let row = sqlx::query_as::<_, Auction>(r#"SELECT * from AUCTIONS WHERE ID = $1"#)
        .bind(auction_id)
        .fetch_optional(con)
        .await?;
    Ok(row)
}

pub async fn fetch_auctions_by_seller(
    con: impl Executor<'_, Database = Postgres>,
    seller: &str,
    status: Option<AuctionStatus>,
) -> Result<Vec<Auction>> {
    let mut qb = QueryBuilder::new("SELECT * from AUCTIONS WHERE SELLER = ");
    qb.push_bind(seller);

    if let Some(status) = status {
        qb.push(" AND STATUS = ");
        qb.push_bind(status);
    }
    qb.push(" ORDER BY LAST_UPDATED_AT DESC");

    let rows = qb.build_query_as::<Auction>().fetch_all(con).await?;
    Ok(rows)
}

pub async fn create_bid(tx: &mut Transaction<'_, Postgres>, b: Bid) -> Result<Bid> {
    let row = sqlx::query_as::<_, Bid>(
        r#"INSERT INTO BIDS
            (AUCTION_ID, BIDDER, BID_PRICE, STATUS)
            VALUES ($1, $2, $3, $4)
            RETURNING *"#,
    )
    .bind(&b.auction_id)
    .bind(&b.bidder)
    .bind(&b.bid_price)
    .bind(&b.status)
    .fetch_one(&mut *tx)
    .await?;
    Ok(row)
}

pub async fn fetch_top_bid(
    con: impl Executor<'_, Database = Postgres>,
    auction_id: &Uuid,
) -> Result<Option<Bid>> {
    let op_row = sqlx::query_as::<_, Bid>(
        r#"SELECT * FROM BIDS WHERE AUCTION_ID = $1 AND STATUS = $2 ORDER BY BID_PRICE DESC LIMIT 1"#,
    )
    .bind(auction_id)
    .bind(&BidStatus::Accepted)
    .fetch_optional(con)
    .await?;
    Ok(op_row)
}

pub async fn fetch_bids_by_auction(
    con: impl Executor<'_, Database = Postgres>,
    auction_id: &Uuid,
) -> Result<Vec<Bid>> {
    let rows = sqlx::query_as::<_, Bid>(
        r#"SELECT * FROM BIDS WHERE AUCTION_ID = $1 AND STATUS IN ($2, $3) ORDER BY BID_PRICE DESC"#,
    )
    .bind(auction_id)
    .bind(&BidStatus::Accepted)
    .bind(&BidStatus::OverTurned)
    .fetch_all(con)
    .await?;
    Ok(rows)
}
