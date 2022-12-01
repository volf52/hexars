use std::net::SocketAddr;

use hexars::{errors::LoginError, get_container, web};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    std::env::set_var("RUST_LOG", "debug");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hexars=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = web::app::create_app().await?;
    let container = get_container!();
    let serv = &container.short_url;

    let e = serv
        .create_short_url("https://www.facebook.com".to_string())
        .await
        .map_err(|_| LoginError::MissingDetails)?;

    println!("Inserted: {:#?}", e);

    let ents = container.short_url.get_all_urls().await;

    println!("All: {:#?}", ents);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Listening on http://localhost:3000");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
