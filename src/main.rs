mod database;
mod entities;
mod routes;
mod schemas;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

#[tokio::main]
async fn main() {
    let db = database::connect()
        .await
        .expect("Database connection failure");
    let state = AppState { db: Arc::new(db) };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000")
        .await
        .expect("Failed to bind server port.");

    let app = routes::api_router().with_state(state);
    axum::serve(listener, app)
        .await
        .expect("Http server failure");
}
