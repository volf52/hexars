use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthStatus {
    status: String,
}

impl HealthStatus {
    pub fn ok() -> Self {
        Self {
            status: "ok".to_string(),
        }
    }
}

pub async fn health() -> Json<HealthStatus> {
    Json(HealthStatus::ok())
}
