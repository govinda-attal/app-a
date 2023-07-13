use crate::prelude::Result as AppResult;
use config::Config;
use serde::{Deserialize, Serialize};

pub fn load(path: &str) -> AppResult<AppConfig> {
    let config = Config::builder()
        .add_source(config::File::with_name(path))
        .add_source(
            config::Environment::with_prefix("APP")
                .separator("_"),
        )
        .build()?;

    let conf: AppConfig = config.try_deserialize()?;

    Ok(conf)
}

#[derive(Debug, Default, Serialize, serde_derive::Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub log: Log,
    pub port: u16,
    pub db: Database,
    pub graceful_shutdown: u8,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Database {
    pub url: String,
}
