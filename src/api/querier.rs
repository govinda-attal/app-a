use super::RpcResult;
use crate::api::v1::querier_server::Querier;
use crate::api::v1::*;
use crate::prelude::*;

use async_trait::async_trait;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct QuerierImpl {}

#[async_trait]
impl Querier for QuerierImpl {
    async fn query_auctions(
        &self,
        request: Request<QueryAuctionsRq>,
    ) -> RpcResult<QueryAuctionsRs> {
        Err(Status::unimplemented("todo!"))
    }
    async fn fetch_auction(&self, request: Request<FetchAuctionRq>) -> RpcResult<FetchAuctionRs> {
        Err(Status::unimplemented("todo!"))
    }
}
