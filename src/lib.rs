pub mod routes;
pub mod system;

use routes::create_route;
use tracing::info;

fn load_address_and_port() -> Result<(String, String), ()> {
    let address = match dotenv::var("ADDRESS") {
        Ok(address) => address,
        Err(e) => {
            tracing::error!("Error loading ADDRESS env variable from .env : {}", e);
            return Err(());
        }
    };

    let port = match dotenv::var("PORT") {
        Ok(port) => port,
        Err(e) => {
            tracing::error!("Error loading ADDRESS env variable from .env : {}", e);
            return Err(());
        }
    };

    Ok((address, port))
}
#[axum::debug_handler]
pub async fn run() {
    let app = create_route();
    let bindings = load_address_and_port().unwrap();
    let url = format!("{}:{}", bindings.0, bindings.1);

    let server = axum::Server::bind(&url.parse().unwrap()).serve(app.into_make_service());
    let addr = server.local_addr();
    info!("Server listening on http://{addr}");
    server.await.unwrap();
}
