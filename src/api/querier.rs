use std::str::FromStr;
use std::sync::Arc;

use super::{validations::SimpleValidation, RpcResult};
use crate::api::v1::*;
use crate::prelude::*;
use crate::{api::v1::querier_server::Querier, db::AuctionQuerier};

use async_trait::async_trait;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct QuerierImpl {
    repo: Arc<dyn AuctionQuerier + Send + Sync>,
}

impl QuerierImpl {
    pub fn new(repo: impl AuctionQuerier + Send + Sync + 'static) -> Self {
        QuerierImpl {
            repo: Arc::new(repo),
        }
    }
}

#[async_trait]
impl Querier for QuerierImpl {
    async fn query_auctions(&self, rq: Request<QueryAuctionsRq>) -> RpcResult<QueryAuctionsRs> {
        let rq = rq.get_ref().validate()?;
        let status = match AuctionStatus::from_i32(rq.status) {
            Some(AuctionStatus::Unspecified) => None,
            v => v,
        };

        let repo = self.repo.clone();
        let rs = QueryAuctionsRs {
            auctions: repo.query_auctions_by_seller(&rq.seller, status).await?,
        };

        Ok(Response::new(rs))
    }
    async fn fetch_auction(&self, rq: Request<FetchAuctionRq>) -> RpcResult<FetchAuctionRs> {
        let auction_id = Uuid::from_str(&rq.get_ref().validate()?.auction_id).unwrap();
        let repo = self.repo.clone();
        let auction = repo.fetch_auction(&auction_id).await?;
        let Some(auction) = auction else {
            return Ok(Response::new(FetchAuctionRs::default()));
        };

        let top_bid = repo.fetch_top_bid(&auction_id).await?;

        let rs = FetchAuctionRs {
            auction: Some(auction),
            top_bid,
        };

        Ok(Response::new(rs))
    }

    async fn query_bids(&self, rq: Request<QueryBidsRq>) -> RpcResult<QueryBidsRs> {
        let auction_id = Uuid::from_str(&rq.get_ref().validate()?.auction_id).unwrap();
        let repo = self.repo.clone();
        let bids = repo.query_bids(&auction_id).await?;

        let rs = QueryBidsRs { bids };

        Ok(Response::new(rs))
    }
}
