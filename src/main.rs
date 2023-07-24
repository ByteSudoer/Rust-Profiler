// #[warn(dead_code)]
mod system;
use axum::Server;
use axum::{
    http::Response,
    response::{Html, IntoResponse},
};
use axum::{routing::get, Router};
use std::env;
use sysinfo::*;
use tracing::*;

fn init() {
    let env_key = "RUST_LOG";
    env::set_var(env_key, "info");
    tracing_subscriber::fmt::init();
}

#[tokio::main]
async fn main() {
    init();
    // let cpuid = raw_cpuid::CpuId::new();
    // let cpu: system::cpu::Cpu = cpuid.into();
    // println!("Cpu : {}", cpu);
    // let json = serde_json::to_string(&cpu).unwrap();
    // println!("Json CPU : {}", json);
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    let disks = sys.disks();
    for d in disks {
        let disk: system::disks::Disk = system::disks::Disk::from_sysinfo_disk(d);
        println!("Disk : {}", disk);
    }
    let router = Router::new()
        .route("/", get(root_get))
        .route("/index.css", get(root_get_css))
        .route("/json", get(get_cpu_info));
    let server = Server::bind(&"127.0.0.1:3000".parse().unwrap()).serve(router.into_make_service());
    // let addr = server.local_addr();
    // info!("Server listening on {addr}");
    // server.await.unwrap();
}

#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/front/index.html")
        .await
        .unwrap();
    info!("HTML file loaded");
    Html(markup)
}
#[axum::debug_handler]
async fn root_get_css() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/front/index.css")
        .await
        .unwrap();
    info!("CSS file loaded");
    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn get_cpu_info() -> impl IntoResponse {
    let cpuid = raw_cpuid::CpuId::new();
    let cpu: system::cpu::Cpu = cpuid.into();
    let json_data = serde_json::to_string(&cpu).unwrap();
    Response::builder()
        .header("content-type", "application/json")
        .body(json_data)
        .unwrap()
}
