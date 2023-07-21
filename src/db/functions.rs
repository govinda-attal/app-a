use crate::prelude::*;
use sqlx::{postgres::PgRow, PgPool, Postgres, Transaction};

use super::model::Auction;

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
