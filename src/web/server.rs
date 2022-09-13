use hexars::{
    app::short_url_service::ShortUrlServ,
    db::{init_db, repos::short_url_repo_sqlx::ShortUrlRepoSqlx, POOL},
    errors::ConfigError,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    dotenv::dotenv().ok();

    init_db().await?;

    let repo = ShortUrlRepoSqlx::default();
    let serv = ShortUrlServ::new(&repo);

    let e = serv
        .create_short_url("https://www.facebook.com".to_string())
        .await;
    println!("Inserted: {:?}", e);

    let ents = serv.get_all_urls().await;

    println!("All: {:?}", ents);

    Ok(())
}
