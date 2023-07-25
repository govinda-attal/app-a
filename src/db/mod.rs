mod functions;
pub mod model;
mod repo;

use crate::api::v1::{AuctionInfo, AuctionRec, AuctionStatus, BidInfo, BidRec};
use crate::prelude::*;

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
use mockall::{automock, mock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuctionMgm {
    async fn create_auction(&self, auction_info: &AuctionInfo) -> Result<AuctionRec>;
    async fn update_auction_status(
        &self,
        auction_id: &Uuid,
        status: AuctionStatus,
    ) -> Result<AuctionRec>;
    async fn create_bid(&self, bid_info: &BidInfo) -> Result<BidRec>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuctionQuerier {
    async fn fetch_top_bid(&self, auction_id: &Uuid) -> Result<Option<BidRec>>;
    async fn fetch_auction(&self, auction_id: &Uuid) -> Result<Option<AuctionRec>>;
    async fn query_auctions_by_seller(
        &self,
        seller: &str,
        status: Option<AuctionStatus>,
    ) -> Result<Vec<AuctionRec>>;
    async fn query_bids(&self, auction_id: &Uuid) -> Result<Vec<BidRec>>;
}

pub fn new_mgm(pool: PgPool) -> impl AuctionMgm + Clone {
    repo::AuctionMgmRepo::new(pool)
}

pub fn new_querier(pool: PgPool) -> impl AuctionQuerier + Clone {
    repo::AuctionQuerierRepo::new(pool)
}
