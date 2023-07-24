use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[axum::debug_handler]
pub async fn return_500() -> Response {
    tracing::error!("500 Internal Server Error");
    (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response()
}
