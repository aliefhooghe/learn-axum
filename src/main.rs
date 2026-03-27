mod auth;
mod database;
mod entities;
mod routes;
mod schemas;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::auth::jwks::{JwksCache, fetch_jwks};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub jwks_cache: Arc<JwksCache>,
    pub issuer_url: String,
    pub realm: String,
}

#[tokio::main]
async fn main() {
    let db = database::connect()
        .await
        .expect("Database connection failure");

    let issuer_url = "http://localhost:8080";
    let realm = "master";

    // TODO: request OIDC at "http://localhost:8080/realms/master/.well-known/openid-configuration",
    // TODO: renew jwks cache
    let jwks_cache = fetch_jwks(issuer_url, realm)
        .await
        .expect("could no retrieve jwks cache.");

    let state = AppState {
        db: Arc::new(db),
        jwks_cache: Arc::new(jwks_cache),
        issuer_url: issuer_url.into(),
        realm: realm.into(),
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000")
        .await
        .expect("Failed to bind server port.");

    let app = routes::api_router(
        "THE_CLIENT_ID",
        "http://localhost:5000/docs/oauth2-redirect.html",
    )
    .with_state(state);

    axum::serve(listener, app)
        .await
        .expect("Http server failure.");
}
