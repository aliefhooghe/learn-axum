mod routes;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000")
        .await
        .expect("Failed to bind server port.");

    let app = routes::api_router();
    axum::serve(listener, app)
        .await
        .expect("Http server failure");
}
