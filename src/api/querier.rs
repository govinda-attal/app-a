use super::RpcResult;
use crate::api::v1::querier_server::Querier;
use crate::api::v1::*;
use crate::prelude::*;

use async_trait::async_trait;
use tonic::{Request, Status};

#[derive(Debug, Default)]
pub struct QuerierImpl {}

#[async_trait]
impl Querier for QuerierImpl {
    async fn query_auctions(
        &self,
        request: Request<QueryAuctionsRq>,
    ) -> RpcResult<QueryAuctionsRs> {
        Err(Error::Unimplemented(format!("query_auctions is not implemented")).into())
    }
    async fn fetch_auction(&self, request: Request<FetchAuctionRq>) -> RpcResult<FetchAuctionRs> {
        Err(Error::Unimplemented(format!("fetch_auction is not implemented")).into())
    }
}
