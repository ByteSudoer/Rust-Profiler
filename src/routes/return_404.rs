use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[axum::debug_handler]
pub async fn return_404() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/front/index_404.html")
        .await
        .unwrap();
    tracing::error!("404 Error Not Found");
    (StatusCode::NOT_FOUND, Html(markup)).into_response()
}
