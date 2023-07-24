use rust_profiler::run;
use std::env;
mod routes;
mod system;

fn init() {
    let env_key = "RUST_LOG";
    env::set_var(env_key, "info");
    tracing_subscriber::fmt::init();
}
#[tokio::main]
async fn main() {
    init();
    run().await
}
