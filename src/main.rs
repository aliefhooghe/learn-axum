mod auth;
mod entities;
mod routes;
mod schemas;
mod settings;

use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;

use crate::auth::jwks::{JwksCache, fetch_jwks};

// Note: AppState is copied to request handler, so copy should be fast
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub jwks_cache: Arc<JwksCache>,
    pub settings: Arc<settings::Settings>,
}

#[tokio::main]
async fn main() {
    let settings = settings::Settings::new().expect("failed to retrieve settings.");
    let db = Database::connect(&settings.database.url)
        .await
        .expect("Database connection failure");

    // TODO: request OIDC at "http://issuer/realms/master/.well-known/openid-configuration",
    // TODO: renew jwks cache
    let jwks_cache = fetch_jwks(&settings.oauth.issuer_url, &settings.oauth.realm)
        .await
        .expect("could no retrieve jwks cache.");

    let state = AppState {
        db: Arc::new(db),
        jwks_cache: Arc::new(jwks_cache),
        settings: Arc::new(settings),
    };

    let listener = tokio::net::TcpListener::bind(&state.settings.server.listen)
        .await
        .expect("Failed to bind server port.");

    let app = routes::api_router(
        &state.settings.oauth.client_id,
        &state.settings.oauth.redirect_url,
    )
    .with_state(state);

    axum::serve(listener, app)
        .await
        .expect("Http server failure.");
}
