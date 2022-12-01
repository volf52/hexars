pub mod repos;

use once_cell::sync::OnceCell;
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, PgPool, Postgres};

use crate::infra::config;
use eyre::ContextCompat;

#[derive(Debug, thiserror::Error)]
pub enum DBConnectionError {
    #[error("POOL not initialized yet")]
    PoolNotInitialized,
    #[error("Failed to acquire Db connection")]
    FailedToAquireConnection,
}

pub static POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn init_db() -> eyre::Result<()> {
    let cfg = config::get_cfg!()?;

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&cfg.db_url)
        .await?;

    POOL.set(pool)
        .map_err(|_| eyre::eyre!("POOL already set"))?;

    Ok(())
}

pub async fn get_db_conn() -> eyre::Result<PoolConnection<Postgres>> {
    let pool = POOL.get().ok_or(DBConnectionError::PoolNotInitialized)?;

    let conn = pool
        .acquire()
        .await
        .map_err(|_| DBConnectionError::FailedToAquireConnection)?;

    Ok(conn)
}
