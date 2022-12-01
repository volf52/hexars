use std::net::SocketAddr;

use axum::{routing::get, Json, Router};
use hexars::{
    db::init_db,
    errors::LoginError,
    infra::{config::init_config, di},
};
use serde::Serialize;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Serialize)]
struct HealthStatus {
    status: String,
}

impl HealthStatus {
    pub fn ok() -> Self {
        Self {
            status: "ok".to_string(),
        }
    }
}

async fn health() -> Json<HealthStatus> {
    Json(HealthStatus::ok())
}

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

    init_config()?;
    init_db().await?;

    let container = di::Container::init();
    let serv = &container.short_url;

    let e = serv
        .create_short_url("https://www.facebook.com".to_string())
        .await
        .map_err(|_| LoginError::MissingDetails)?;

    println!("Inserted: {:#?}", e);

    let ents = &container.short_url.get_all_urls().await;

    println!("All: {:#?}", ents);

    let app = Router::new()
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http()); // can customize

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Listening on http://localhost:3000");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
