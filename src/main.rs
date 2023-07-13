#![allow(unused)]

mod api;
mod cmd;
mod config;
mod db;
mod error;
mod prelude;

use crate::prelude::*;
use futures_util::FutureExt;
use std::net::SocketAddr;
use tokio::sync::oneshot::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio::signal;

use sqlx::PgPool;
use structured_logger::{async_json::new_writer, Builder as LogBuilder};
use tokio::io;
use tokio::time::{sleep, Duration};
use tonic::transport::Server as TonicServer;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cmd::parse();
    let conf = config::load(&args.config_path)?;
    let shutdown_seconds = conf.graceful_shutdown;
    LogBuilder::with_level(&conf.log.level)
        .with_default_writer(new_writer(io::stdout()))
        .init();

    let (main_tx, app_rx) = oneshot::channel::<()>();
    let (app_tx, main_rx) = oneshot::channel::<()>();

    let app_task = tokio::spawn(run_app(conf, app_tx, app_rx));
    log::info!("starting application...");
    tokio::select! {
        _ = signal::ctrl_c() => {
            let _ = main_tx.send(());
            sleep(Duration::from_secs(shutdown_seconds as u64)).await;
            main_rx.await?;
        },
        task_rs = app_task => {
            let rs = task_rs?;
            if rs.is_err() {
                let e = rs.unwrap_err();
                log::error!("application stopped unexpectedly {}", e);
                return Err(e);
            };
        }
    }
    Ok(())
}

async fn run_app(conf: config::AppConfig, tx: Sender<()>, rx: Receiver<()>) -> Result<()> {
    let addr: SocketAddr = format!("0.0.0.0:{}", conf.port).parse()?;

    let pool = PgPool::connect(&conf.db.url).await?;
    log::info!("running db migrations");
    sqlx::migrate!("db/migrations").run(&pool).await?;

    log::info!(
        "running application on port {} and db {}",
        conf.port,
        conf.db.url
    );

    let repo = db::new_repo(pool);

    TonicServer::builder()
        .add_service(api::spec_service()?)
        .add_service(api::processor_service(repo))
        .add_service(api::querier_service())
        .serve_with_shutdown(addr, rx.map(drop))
        .await?;

    
    if let Err(_) = tx.send(()) {
        log::warn!("main thread dropped receiever");
    }
    log::info!("application shutdown gracefully");
    Ok(())
}
