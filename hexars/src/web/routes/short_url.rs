use axum::{extract::State, routing::post, Json, Router};

use crate::{
    domain::short_url_entity::ShortUrl,
    infra::di::{get_container, Container},
    web::response::HttpResult,
};

async fn shorten_url(State(container): State<&Container>) -> HttpResult<Json<ShortUrl>> {
    let ent = container
        .short_url
        .create_short_url("http://www.google.com".to_string())
        .await?;

    Ok(Json(ent))
}

async fn fetch_all(State(container): State<&Container>) -> HttpResult<Json<Vec<ShortUrl>>> {
    let urls = container.short_url.get_all_urls().await?;

    // Err(AppError::NotLoggedIn.into())
    Ok(Json(urls))
}

pub fn create_short_url_router() -> axum::Router {
    let container = get_container!();

    Router::new()
        .route("/", post(shorten_url).get(fetch_all))
        .with_state(container)
}
