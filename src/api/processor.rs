use crate::prelude::*;
use std::sync::Arc;
use super::RpcResult;
use crate::api::v1::*;
use crate::db::AuctionMgm;
use crate::{api::v1::processor_server::Processor, db};

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
    async fn new_auction(&self, rq: Request<NewAuctionRq>) -> RpcResult<NewAuctionRs> {
        // Will revist this for input validations in subsequent iterations
        let info = rq.into_inner().info.unwrap_or_default();

        let rec = self.repo.clone().create_auction(&info).await?;
        
        Ok(Response::new(NewAuctionRs { rec: Some(rec) }))
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
    async fn test_new_auction_simple() {
        let info = AuctionInfo {
            item: "item-x".into(),
            description: "item x description".into(),
            seller: "seller".into(),
            start_price: 100,
            ..Default::default()
        };

        let auction_rec = &AuctionRec {
            id: "id".into(),
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
            .new_auction(Request::new(NewAuctionRq {
                info: Some(info.to_owned()),
            }))
            .await
            .unwrap();
        assert_eq!(auction_rec.to_owned(), rs.into_inner().rec.unwrap(),);
    }
}
