use super::validations::*;
use super::RpcResult;
use crate::api::v1::*;
use crate::db::AuctionMgm;
use crate::prelude::*;
use crate::{api::v1::processor_server::Processor, db};
use std::sync::Arc;

use async_trait::async_trait;
use tonic::{Request, Response, Status};

pub struct ProcessorImpl {
    repo: Arc<dyn AuctionMgm + Send + Sync>,
}

impl ProcessorImpl {
    pub fn new(repo: impl AuctionMgm + Send + Sync + 'static) -> Self {
        ProcessorImpl {
            repo: Arc::new(repo),
        }
    }
}

#[async_trait]
impl Processor for ProcessorImpl {
    async fn draft_auction(&self, rq: Request<DraftAuctionRq>) -> RpcResult<DraftAuctionRs> {
        let info = rq.get_ref().validate()?.info.as_ref().unwrap();

        let rec = self.repo.clone().create_auction(&info).await?;

        Ok(Response::new(DraftAuctionRs { rec: Some(rec) }))
    }
    async fn start_auction(&self, rq: Request<StartAuctionRq>) -> RpcResult<StartAuctionRs> {
        Err(Error::Unimplemented(format!("open_auction is not implemented")).into())
    }
    async fn new_bid(&self, rq: Request<NewBidRq>) -> RpcResult<NewBidRs> {
        Err(Error::Unimplemented(format!("new_bid is not implemented")).into())
    }
    async fn close_auction(&self, rq: Request<CloseAuctionRq>) -> RpcResult<CloseAuctionRs> {
        Err(Error::Unimplemented(format!("close_auction is not implemented")).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::db::MockAuctionMgm;
    use mockall::predicate::eq;
    use prost_types::Timestamp;

    use super::*;

    #[tokio::test]
    async fn test_draft_auction_bad_request() {
        let info = AuctionInfo::default();
        let mut mock = MockAuctionMgm::new();
        let processor = ProcessorImpl::new(mock);
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

        let mut mock = MockAuctionMgm::new();
        mock.expect_create_auction()
            .with(eq(info.clone()))
            .times(1)
            .returning(move |info: &AuctionInfo| -> Result<AuctionRec> {
                Ok(mock_rec_out.clone())
            });

        let processor = ProcessorImpl::new(mock);
        let rs = processor
            .draft_auction(Request::new(DraftAuctionRq {
                info: Some(info.to_owned()),
            }))
            .await
            .unwrap();
        assert_eq!(auction_rec.to_owned(), rs.into_inner().rec.unwrap(),);
    }
}
