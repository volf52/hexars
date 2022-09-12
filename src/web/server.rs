use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await?;

    Ok(())
}
