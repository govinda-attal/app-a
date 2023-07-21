use crate::api::v1::{AuctionInfo, AuctionRec, BidInfo, BidRec};
use crate::prelude::*;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use super::functions::create_auction;
use super::model::Auction;
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
        let input = Auction::try_from(auction_info)?;
        let mut tx = self.pool.clone().begin().await?;
        let row = create_auction(&mut tx, input).await?;
        tx.commit().await?;
        Ok(AuctionRec::from(&row))
    }

    async fn create_bid(&self, auction_id: &str, bid_info: &BidInfo) -> Result<BidRec> {
        Err(Error::Unimplemented("create_bid is not implemented".into()))
    }
}
