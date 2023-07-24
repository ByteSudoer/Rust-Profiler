mod cpu_info;
mod disk_info;
mod full_system;
mod get_roots;
mod return_404;
mod return_500;

use axum::{routing::get, Router};
use cpu_info::get_cpu_info;
use disk_info::get_disk_info;
use full_system::get_full_system_info;
use get_roots::{root_get, root_get_css};
use return_404::return_404;
use return_500::return_500;

#[allow(dead_code)]
pub fn create_route() -> Router {
    Router::new()
        .route("/return_500", get(return_500))
        .route("/return_404", get(return_404))
        .route("/index.css", get(root_get_css))
        .route("/cpu_info", get(get_cpu_info))
        .route("/disk_info", get(get_disk_info))
        .route("/full_system", get(get_full_system_info))
        .route("/", get(root_get))
}
