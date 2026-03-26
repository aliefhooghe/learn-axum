mod routes;

use axum::{Router, routing::get};
use routes::router1;

fn create_app() -> Router {
    Router::new()
        .route("/", get(|| async { "hello" }))
        .merge(router1::create_router())
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    let app = create_app();
    axum::serve(listener, app).await.unwrap();
}
