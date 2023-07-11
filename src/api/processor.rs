use super::RpcResult;
use crate::api::v1::processor_server::Processor;
use crate::api::v1::*;
use crate::prelude::*;

use async_trait::async_trait;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ProcessorImpl {}

#[async_trait]
impl Processor for ProcessorImpl {
    async fn new_auction(&self, rq: Request<NewAuctionRq>) -> RpcResult<NewAuctionRs> {
        Err(Status::unimplemented("todo!"))
    }
    async fn new_bid(&self, rq: Request<NewBidRq>) -> RpcResult<NewBidRs> {
        Err(Status::unimplemented("todo!"))
    }
    async fn close_auction(&self, rq: Request<CloseAuctionRq>) -> RpcResult<CloseAuctionRs> {
        Err(Status::unimplemented("todo!"))
    }
}
