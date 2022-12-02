use crate::errors::ConfigError;
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use std::env;

pub struct Config {
    pub db_url: String,
    pub port: u16,
}

pub static APP_CONFIG: OnceCell<Config> = OnceCell::new();

pub fn init_config() -> std::result::Result<(), ConfigError> {
    if cfg!(debug_assertions) {
        tracing::debug!("Loading env from .env.local");
        dotenvy::from_filename(".env.local").ok();
    } else {
        tracing::debug!("Loading env from .env (release)");
        dotenv().ok();
    }

    let db_url = env::var("DATABASE_URL").map_err(|_| ConfigError::DatabaseUrlNotFound)?;

    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().map_err(|_| ConfigError::InvalidPort(p)),
        Err(_) => Ok(3000),
    }?;

    let cfg = Config { db_url, port };

    APP_CONFIG
        .set(cfg)
        .map_err(|_| ConfigError::AlreadyInitialized)?;

    Ok(())
}

#[macro_export]
macro_rules! get_cfg {
    () => {{
        use eyre::ContextCompat;

        $crate::infra::config::APP_CONFIG
            .get()
            .wrap_err("Config not yet initialized")
    }};
}

pub(crate) use get_cfg;
