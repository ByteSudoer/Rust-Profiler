use crate::system;
use axum::{http::Response, response::IntoResponse};

#[axum::debug_handler]
pub async fn get_disk_info() -> impl IntoResponse {
    tracing::info!("Disk info requested");
    let disks = system::disks::Disks::new();
    let json_data = serde_json::to_string(&disks).unwrap();
    Response::builder()
        .header("content-type", "application/json")
        .body(json_data)
        .unwrap()
}
