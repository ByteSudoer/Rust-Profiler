mod return_500;
mod get_roots;

use axum::{routing::get, Router};
use return_500::return_500;
use get_roots::{root_get,root_get_css};

pub fn create_route() -> Router {
    Router::new().route("/return_500", get(return_500))
    .route("/index.css", get(root_get_css))
    .route("/",get(root_get))
}
