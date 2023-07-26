use axum::{
    http::Response,
    response::{Html, IntoResponse},
};

#[axum::debug_handler]
pub async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/front/index.html")
        .await
        .unwrap();
    tracing::info!("HTML file loaded");
    Html(markup)
}

#[axum::debug_handler]
pub async fn root_get_css() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/front/index.css")
        .await
        .unwrap();
    tracing::info!("CSS file loaded");
    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
pub async fn root_get_mjs() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/front/index.mjs")
        .await
        .unwrap();

    tracing::info!("MJS file loaded");
    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}
