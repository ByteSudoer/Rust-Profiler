use axum::Server;
use axum::{routing::get, Router};
use std::env;
use tracing::*;

#[tokio::main]
async fn main() {
    let env_key = "RUST_LOG";
    env::set_var(env_key, "trace");
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let server = Server::bind(&"127.0.0.1:3000".parse().unwrap()).serve(app.into_make_service());
    let addr = server.local_addr();
    info!("Server listening on {addr}");
    server.await.unwrap();
}
