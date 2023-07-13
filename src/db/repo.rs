use crate::api::v1::{AuctionInfo, AuctionRec, BidInfo, BidRec};
use crate::prelude::*;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use super::AuctionMgm;

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
        Err(Error::Unimplemented("create_auction is not implemented".into()))
    }

    async fn create_bid(&self, auction_id: &str, bid_info: &BidInfo) -> Result<BidRec> {
        Err(Error::Unimplemented("create_bid is not implemented".into()))
    }
}
