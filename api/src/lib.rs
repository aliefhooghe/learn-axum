mod auth;
mod logging;
mod routes;
mod schemas;
mod settings;

use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::auth::jwks::{JwksCache, refresh_jwks_task};

// Note: AppState is copied to request handler, so copy should be fast
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub jwks_cache: Arc<RwLock<JwksCache>>,
    pub settings: Arc<settings::Settings>,
}

#[tokio::main]
pub async fn main() {
    let settings = settings::Settings::new().expect("failed to retrieve settings.");
    logging::init(&settings);
    tracing::info!("starting ...");

    tracing::info!("connect to database.");
    let db = Database::connect(&settings.database.url)
        .await
        .expect("Database connection failure");

    // TODO: request OIDC at "http://issuer/realms/master/.well-known/openid-configuration",
    let state = AppState {
        db: Arc::new(db),
        jwks_cache: Arc::new(RwLock::new(JwksCache::new())),
        settings: Arc::new(settings),
    };

    let task_state = state.clone();
    tokio::spawn(async {
        refresh_jwks_task(task_state).await;
    });

    tracing::info!("listening on {}.", &state.settings.server.listen);
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
