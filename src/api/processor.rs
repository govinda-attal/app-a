use super::validations::*;
use super::RpcResult;
use crate::api::v1::*;
use crate::db::AuctionMgm;
use crate::db::AuctionQuerier;
use crate::prelude::*;
use crate::{api::v1::processor_server::Processor, db};
use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct ProcessorImpl {
    mgm_repo: Arc<dyn AuctionMgm + Send + Sync>,
    qry_repo: Arc<dyn AuctionQuerier + Send + Sync>,
}

impl ProcessorImpl {
    pub fn new(
        mgm_repo: impl AuctionMgm + Send + Sync + 'static,
        qry_repo: impl AuctionQuerier + Send + Sync + 'static,
    ) -> Self {
        ProcessorImpl {
            mgm_repo: Arc::new(mgm_repo),
            qry_repo: Arc::new(qry_repo),
        }
    }
}

#[async_trait]
impl Processor for ProcessorImpl {
    async fn draft_auction(&self, rq: Request<DraftAuctionRq>) -> RpcResult<DraftAuctionRs> {
        let info = rq.get_ref().validate()?.info.as_ref().unwrap();

        let rec = self.mgm_repo.clone().create_auction(info).await?;

        Ok(Response::new(DraftAuctionRs { rec: Some(rec) }))
    }
    async fn start_auction(&self, rq: Request<StartAuctionRq>) -> RpcResult<StartAuctionRs> {
        let auction_id = Uuid::from_str(&rq.get_ref().validate()?.auction_id).unwrap();
        let rec = self
            .mgm_repo
            .clone()
            .update_auction_status(&auction_id, AuctionStatus::Open)
            .await?;
        Ok(Response::new(StartAuctionRs { auction: Some(rec) }))
    }
    async fn new_bid(&self, rq: Request<NewBidRq>) -> RpcResult<NewBidRs> {
        let info = rq.get_ref().validate()?.info.as_ref().unwrap();

        let rec = self.mgm_repo.clone().create_bid(info).await?;

        Ok(Response::new(NewBidRs { rec: Some(rec) }))
    }
    async fn close_auction(&self, rq: Request<CloseAuctionRq>) -> RpcResult<CloseAuctionRs> {
        let auction_id = Uuid::from_str(&rq.get_ref().validate()?.auction_id).unwrap();

        let mgm_repo = self.mgm_repo.clone();

        let qry_repo = self.qry_repo.clone();

        let rec = mgm_repo
            .update_auction_status(&auction_id, AuctionStatus::Closed)
            .await?;
        Ok(Response::new(CloseAuctionRs {
            auction: Some(rec),
            top_bid: qry_repo.fetch_top_bid(&auction_id).await?,
        }))
    }

    async fn cancel_auction(&self, rq: Request<CancelAuctionRq>) -> RpcResult<CancelAuctionRs> {
        let auction_id = Uuid::from_str(&rq.get_ref().validate()?.auction_id).unwrap();
        let rec = self
            .mgm_repo
            .clone()
            .update_auction_status(&auction_id, AuctionStatus::Cancelled)
            .await?;
        Ok(Response::new(CancelAuctionRs { auction: Some(rec) }))
    }
}

#[cfg(test)]
mod tests {
    use crate::db::{MockAuctionMgm, MockAuctionQuerier};
    use mockall::predicate::eq;
    use prost_types::Timestamp;

    use super::*;

    #[tokio::test]
    async fn test_draft_auction_bad_request() {
        let info = AuctionInfo::default();
        let mut mgm_mock = MockAuctionMgm::new();
        let mut qry_mock = MockAuctionQuerier::new();
        let processor = ProcessorImpl::new(mgm_mock, qry_mock);
        let rs = processor
            .draft_auction(Request::new(DraftAuctionRq {
                info: Some(info.to_owned()),
            }))
            .await
            .unwrap_err();

        assert_eq!(rs.code(), tonic::Code::InvalidArgument);
        assert_eq!(rs.message(), "info.item is required");
    }

    #[tokio::test]
    async fn test_draft_auction_simple() {
        let info = AuctionInfo {
            item: f!("item-x"),
            description: f!("item x description"),
            seller: f!("seller"),
            start_price: 100,
            ..Default::default()
        };

        let auction_rec = &AuctionRec {
            id: f!("id"),
            created_at: Some(Timestamp::default()),
            updated_at: Some(Timestamp::default()),
            info: Some(info.clone()),
            status: AuctionStatus::Open as i32,
        };

        let mock_rec_out = auction_rec.clone();

        let mut mgm_mock = MockAuctionMgm::new();
        let mut qry_mock = MockAuctionQuerier::new();
        mgm_mock
            .expect_create_auction()
            .with(eq(info.clone()))
            .times(1)
            .returning(move |info: &AuctionInfo| -> Result<AuctionRec> {
                Ok(mock_rec_out.clone())
            });

        let processor = ProcessorImpl::new(mgm_mock, qry_mock);
        let rs = processor
            .draft_auction(Request::new(DraftAuctionRq {
                info: Some(info.to_owned()),
            }))
            .await
            .unwrap();
        assert_eq!(auction_rec.to_owned(), rs.into_inner().rec.unwrap(),);
    }
}
