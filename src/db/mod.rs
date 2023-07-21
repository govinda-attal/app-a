mod functions;
pub mod model;
mod repo;

use crate::api::v1::{AuctionInfo, AuctionRec, BidInfo, BidRec};
use crate::prelude::*;

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[cfg(test)]
use mockall::{automock, mock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuctionMgm {
    async fn create_auction(&self, auction_info: &AuctionInfo) -> Result<AuctionRec>;
    async fn create_bid(&self, auction_id: &str, bid_info: &BidInfo) -> Result<BidRec>;
}

pub fn new_repo(pool: PgPool) -> impl AuctionMgm {
    repo::AuctionMgmRepo::new(pool)
}
