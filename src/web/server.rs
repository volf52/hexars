use hexars::{db::init_db, errors::LoginError, infra::di};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    dotenv::dotenv().ok();

    init_db().await?;

    let container = di::Container::init();
    let serv = container.short_url;

    let e = serv
        .create_short_url("https://www.facebook.com".to_string())
        .await
        .map_err(|_| LoginError::MissingDetails)?;

    println!("Inserted: {:#?}", e);

    let ents = serv.get_all_urls().await;

    println!("All: {:#?}", ents);

    Ok(())
}
