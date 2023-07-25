use crate::api::v1::{AuctionInfo, AuctionRec, AuctionStatus, BidInfo, BidRec};
use crate::prelude::*;
use async_trait::async_trait;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use super::functions::{
    create_auction, create_bid, fetch_auction, fetch_auctions_by_seller, fetch_bids_by_auction,
    fetch_top_bid, overturn_bid, update_auction_status,
};
use super::model::{Auction, AuctionStatus as DBAuctionStatus, Bid, BidStatus as DBBidStatus};
use super::{AuctionMgm, AuctionQuerier};

#[derive(Debug, Clone)]
pub struct AuctionMgmRepo {
    pool: Arc<PgPool>,
}

impl AuctionMgmRepo {
    pub fn new(pool: PgPool) -> Self {
        AuctionMgmRepo {
            pool: Arc::new(pool),
        }
    }
}

#[async_trait]
impl AuctionMgm for AuctionMgmRepo {
    async fn create_auction(&self, auction_info: &AuctionInfo) -> Result<AuctionRec> {
        let input = Auction::try_from(auction_info)?;
        let mut tx = self.pool.clone().begin().await?;
        let row = create_auction(&mut tx, input).await?;
        tx.commit().await?;
        Ok(AuctionRec::from(&row))
    }

    async fn update_auction_status(
        &self,
        auction_id: &Uuid,
        status: AuctionStatus,
    ) -> Result<AuctionRec> {
        let status = DBAuctionStatus::try_from(&status)?;
        let mut tx = self.pool.clone().begin().await?;
        let row = update_auction_status(&mut tx, auction_id, status).await?;
        tx.commit().await?;
        Ok(AuctionRec::from(&row))
    }

    async fn create_bid(&self, bid_info: &BidInfo) -> Result<BidRec> {
        let mut bid = Bid::try_from(bid_info)?;
        let auction_id = &bid.auction_id;

        let mut tx = self.pool.clone().begin().await?;

        let auction = fetch_auction(&mut tx, auction_id).await?;
        let Some(auction) = auction else {
            return Err(Error::BadRquest(
                "auction not found".to_string()))?;
        };
        if auction.status != DBAuctionStatus::Open {
            return Err(Error::BadRquest(f!(
                "auction for {} is not in open state",
                auction.item
            )))?;
        }

        if auction.start_price > bid.bid_price {
            bid.status = DBBidStatus::Rejected;
        }

        let top_bid = fetch_top_bid(&mut tx, auction_id).await?;
        if let Some(top_bid) = top_bid {
            if top_bid.bid_price >= bid.bid_price {
                bid.status = DBBidStatus::Rejected;
            } else {
                _ = overturn_bid(&mut tx, &top_bid.id.unwrap()).await?;
            }
        }

        if bid.status == DBBidStatus::Empty {
            bid.status = DBBidStatus::Accepted;
        }

        let row = create_bid(&mut tx, bid).await?;
        tx.commit().await?;

        Ok(BidRec::from(&row))
    }
}

#[derive(Debug, Clone)]
pub struct AuctionQuerierRepo {
    pool: Arc<PgPool>,
}

impl AuctionQuerierRepo {
    pub fn new(pool: PgPool) -> Self {
        AuctionQuerierRepo {
            pool: Arc::new(pool),
        }
    }
}

#[async_trait]
impl AuctionQuerier for AuctionQuerierRepo {
    async fn fetch_top_bid(&self, auction_id: &Uuid) -> Result<Option<BidRec>> {
        let row = fetch_top_bid(self.pool.as_ref(), auction_id).await?;
        Ok(row.map(|row| BidRec::from(&row)))
    }

    async fn fetch_auction(&self, auction_id: &Uuid) -> Result<Option<AuctionRec>> {
        let auction = fetch_auction(self.pool.as_ref(), auction_id).await?;
        Ok(auction.map(|a| AuctionRec::from(&a)))
    }

    async fn query_auctions_by_seller(
        &self,
        seller: &str,
        status: Option<AuctionStatus>,
    ) -> Result<Vec<AuctionRec>> {
        let status = status.map(|s| DBAuctionStatus::try_from(&s).unwrap());
        let auctions = fetch_auctions_by_seller(self.pool.as_ref(), seller, status).await?;
        Ok(auctions
            .into_iter()
            .map(|a| AuctionRec::from(&a))
            .collect::<_>())
    }

    async fn query_bids(&self, auction_id: &Uuid) -> Result<Vec<BidRec>> {
        let bids = fetch_bids_by_auction(self.pool.as_ref(), auction_id).await?;
        Ok(bids.iter().map(|b| BidRec::from(b)).collect::<_>())
    }
}
