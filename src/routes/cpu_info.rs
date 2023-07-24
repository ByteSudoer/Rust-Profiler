use axum::{http::Response, response::IntoResponse};

use crate::system;
#[axum::debug_handler]
pub async fn get_cpu_info() -> impl IntoResponse {
    tracing::info!("Cpu info requested");
    let cpuid = raw_cpuid::CpuId::new();
    let cpu: system::cpu::Cpu = cpuid.into();
    let json_data = serde_json::to_string(&cpu).unwrap();
    Response::builder()
        .header("content-type", "application/json")
        .body(json_data)
        .unwrap()
}
