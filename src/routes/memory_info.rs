use crate::system;
use axum::{http::Response, response::IntoResponse};

#[axum::debug_handler]
pub async fn get_memory_info() -> impl IntoResponse {
    tracing::info!("Memory info requested");
    let memory = system::memory::Memory::new();
    let json_data = serde_json::to_string(&memory).unwrap();
    Response::builder()
        .header("content-type", "application/json")
        .body(json_data)
        .unwrap()
}
