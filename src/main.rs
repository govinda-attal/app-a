#![allow(unused)]

mod api;
mod cmd;
mod config;
mod error;
mod prelude;

use crate::prelude::*;
use std::path::PathBuf;

use structured_logger::{async_json::new_writer, Builder as LogBuilder};
use tokio::io;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cmd::parse();
    let conf = config::load(PathBuf::from(args.config_path))?;

    LogBuilder::with_level(&conf.log.level)
        .with_default_writer(new_writer(io::stdout()))
        .init();

    let addr = format!("127.0.0.1:{}", conf.port).parse()?;

    log::info!("running server on port {}", conf.port);

    Server::builder()
        .add_service(api::spec_service()?)
        .add_service(api::processor_server())
        .add_service(api::querier_server())
        .serve(addr)
        .await?;
    
    Ok(())
}
