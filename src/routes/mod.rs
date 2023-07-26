mod cpu_info;
mod disk_info;
mod full_system;
mod get_roots;
mod realtime_cpu;
mod return_404;
mod return_500;
mod users_info;

use axum::{routing::get, Router};
use cpu_info::get_cpu_info;
use disk_info::get_disk_info;
use full_system::get_full_system_info;
use get_roots::{root_get, root_get_css, root_get_mjs};
use return_404::return_404;
use return_500::return_500;
use users_info::get_one_user_by_name;
use users_info::get_one_user_by_uid;
use users_info::get_users_info;

#[allow(dead_code)]
pub fn create_route() -> Router {
    Router::new()
        .route("/", get(root_get))
        .route("/return_500", get(return_500))
        .route("/return_404", get(return_404))
        .route("/index.css", get(root_get_css))
        .route("/index.mjs", get(root_get_mjs))
        .route("/cpu_info", get(get_cpu_info))
        .route("/disk_info", get(get_disk_info))
        .route("/users_info", get(get_users_info))
        .route("/user_info/find/:user_name", get(get_one_user_by_name))
        .route("/user_info/find_uid/:user_uid", get(get_one_user_by_uid))
        .route("/full_system", get(get_full_system_info))
        .merge(realtime_cpu::create_realtime_cpu_route())
}
