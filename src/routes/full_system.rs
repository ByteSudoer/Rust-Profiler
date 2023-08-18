use crate::system;
use axum::{http::Response, response::IntoResponse};

#[allow(dead_code)]
#[axum::debug_handler]
pub async fn get_full_system_info() -> impl IntoResponse {
    tracing::info!("Full System info requested");
    let full_system = system::full_system::FullSystem::new();
    let json_data = serde_json::to_string(&full_system).unwrap();
    Response::builder()
        .header("Content-Type", "application/json")
        .body(json_data)
        .unwrap()
}
