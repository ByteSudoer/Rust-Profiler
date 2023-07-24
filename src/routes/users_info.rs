use crate::system;
use axum::{
    extract::Path,
    http::{Response, StatusCode},
    response::IntoResponse,
};

#[axum::debug_handler]
pub async fn get_users_info() -> impl IntoResponse {
    tracing::info!("Users info requested");
    let users = system::users::Users::new();
    let json_data = serde_json::to_string(&users).unwrap();
    Response::builder()
        .header("content-type", "application/json")
        .body(json_data)
        .unwrap()
}

#[axum::debug_handler]
pub async fn get_one_user_by_name(Path(user_name): Path<String>) -> impl IntoResponse {
    tracing::info!("Find user with name : {} requested", user_name);
    let users_all = system::users::Users::new();
    match users_all.find_one_user_by_name(&user_name.to_string()) {
        Some(user) => {
            tracing::info!("User found : {}", user);
            let json_data = serde_json::to_string(&user).unwrap();
            Response::builder()
                .header("content-type", "application/json")
                .body(json_data)
                .unwrap()
        }
        _ => {
            tracing::error!("User with name : {} not found", user_name.to_string());
            Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header("Location", "/return_404")
                .body("".into())
                .unwrap()
        }
    }
}

#[axum::debug_handler]
pub async fn get_one_user_by_uid(Path(user_uid): Path<u32>) -> impl IntoResponse {
    tracing::info!("Find user with UID {} requested", user_uid.to_string());
    let users_all = system::users::Users::new();
    match users_all.find_one_user_by_uid(user_uid) {
        Some(user) => {
            tracing::info!("User found : {}", user);
            let json_data = serde_json::to_string(&user).unwrap();
            Response::builder()
                .header("content-type", "application/json")
                .body(json_data)
                .unwrap()
        }
        _ => {
            tracing::error!("User with UID {} not found", user_uid.to_string());
            Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header("Location", "/return_404")
                .body("".into())
                .unwrap()
        }
    }
}
