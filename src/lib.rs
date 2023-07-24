pub mod routes;
pub mod system;

use routes::create_route;
use tracing::info;

#[axum::debug_handler]
pub async fn run() {
    let app = create_route();
    let server =
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service());
    let addr = server.local_addr();
    info!("Server listening on {addr}");
    server.await.unwrap();
}
