pub mod repos;

use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::infra::config;

pub static POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn init_db() -> color_eyre::Result<()> {
    let cfg = config::APP_CONFIG.get().expect("Config Not Initialized");

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&cfg.db_url)
        .await?;

    POOL.set(pool).expect("unable to set DB POOL");

    Ok(())
}
