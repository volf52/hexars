pub mod repos;

use once_cell::sync::OnceCell;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::infra::config;

pub static POOL: OnceCell<SqlitePool> = OnceCell::new();

pub async fn init_db() -> color_eyre::Result<()> {
    let db_url = std::env::var("DATABASE_URL")?;
    let cfg = config::APP_CONFIG.get().unwrap();

    let pool = SqlitePoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await?;

    POOL.set(pool).expect("unable to set POOL");

    Ok(())
}
