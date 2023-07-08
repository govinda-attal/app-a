// #![allow(unused)]

mod cmd;
mod config;
mod error;
mod prelude;

use crate::prelude::*;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

use structured_logger::{async_json::new_writer, Builder as LogBuilder};
use tokio::io;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cmd::parse();
    let conf = config::load(PathBuf::from(args.config_path))?;

    LogBuilder::with_level(&conf.log.level)
        .with_default_writer(new_writer(io::stdout()))
        .init();

    log::info!(conf = log::as_serde!(conf); "configuration");

    sleep(Duration::from_millis(100)).await;

    Ok(())
}
