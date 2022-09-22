pub mod repos;

// use refinery::embed_migrations;
// embed_migrations!("src/infra/db/migrations");

use once_cell::sync::OnceCell;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub static POOL: OnceCell<SqlitePool> = OnceCell::new();

pub async fn init_db() -> color_eyre::Result<()> {
    let db_url = std::env::var("DATABASE_URL")?;

    let pool = SqlitePoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await?;

    POOL.set(pool).expect("unable to set POOL");

    Ok(())
}
