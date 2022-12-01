use crate::errors::ConfigError;
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use std::env;

pub struct Config {
    pub db_url: String,
}

pub static APP_CONFIG: OnceCell<Config> = OnceCell::new();

pub fn init_config() -> std::result::Result<(), ConfigError> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").map_err(|_| ConfigError::DatabaseUrlNotFound)?;

    let cfg = Config { db_url };

    APP_CONFIG
        .set(cfg)
        .map_err(|_| ConfigError::AlreadyInitialized)?;

    Ok(())
}

// todo: maybe add import for eyre::Context thing
macro_rules! get_cfg {
    () => {
        crate::infra::config::APP_CONFIG
            .get()
            .wrap_err("Config not yet initialized")
    };
}

pub(crate) use get_cfg;
