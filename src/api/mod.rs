use crate::prelude::*;
mod processor;
mod querier;

use tonic_reflection::server::{ServerReflection, ServerReflectionServer};

pub mod v1 {
    tonic::include_proto!("auction.v1");
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("auction_v1_descriptor");
}

// spec_service returns reflection server to allow reading proto definition at runtime.
pub fn spec_service() -> Result<ServerReflectionServer<impl ServerReflection>> {
    let spec = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(v1::FILE_DESCRIPTOR_SET)
        .build()?;
    Ok(spec)
}

pub type RpcResult<T> = std::result::Result<tonic::Response<T>, tonic::Status>;

use processor::ProcessorImpl;
use querier::QuerierImpl;
use v1::processor_server::*;
use v1::querier_server::*;

pub fn processor_service() -> ProcessorServer<impl Processor> {
    ProcessorServer::new(ProcessorImpl {})
}

pub fn querier_service() -> QuerierServer<impl Querier> {
    QuerierServer::new(QuerierImpl {})
}
