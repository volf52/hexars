use super::routes;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::infra::{config, db, di};

pub async fn create_app() -> eyre::Result<axum::Router> {
    config::init_config()?;
    db::init_db().await?;

    di::Container::init();

    let surl_router = routes::short_url::create_short_url_router();

    let app = Router::new()
        .route("/health", get(routes::health::health))
        .nest("/url", surl_router)
        .layer(TraceLayer::new_for_http()); // can customize

    Ok(app)
}
