use hexars::{db::init_db, errors::LoginError, infra::di};
use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, web::Json, EndpointExt, Route, Server,
};
use serde::Serialize;

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

#[handler]
async fn health() -> Json<HealthStatus> {
    Json(HealthStatus::ok())
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    std::env::set_var("RUST_LOG", "poem=debug");
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();

    init_db().await?;

    let container = di::Container::init();
    let serv = &container.short_url;

    // let e = serv
    //     .create_short_url("https://www.facebook.com".to_string())
    //     .await
    //     .map_err(|_| LoginError::MissingDetails)?;
    //
    // println!("Inserted: {:#?}", e);
    //
    // let ents = &container.short_url.get_all_urls().await;
    //
    // println!("All: {:#?}", ents);

    let app = Route::new().at("/health", get(health)).with(Tracing);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    Ok(())
}
