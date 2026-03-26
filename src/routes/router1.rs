use axum::{Router, routing::get};

pub fn create_router() -> Router {
    Router::new()
        .route("/a", get(|| async { "hello a" }))
        .route("/b", get(|| async { "hello b" }))
}
