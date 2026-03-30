use axum::{response::Redirect, routing::get};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::{Config, SwaggerUi, oauth};

use crate::AppState;

mod auth;
mod openapi;
mod resources;
mod users;

use openapi::ApiDoc;

fn swagger_ui(api: utoipa::openapi::OpenApi, client_id: &str) -> SwaggerUi {
    let openapi_path = "/docs/openapi.json";
    let swagger_config = Config::with_oauth_config(
        [openapi_path],
        oauth::Config::new()
            .client_id(client_id)
            .scopes(vec!["email".into(), "profile".into(), "openid".into()])
            .use_pkce_with_authorization_code_grant(true),
    );
    SwaggerUi::new("/docs")
        .url(openapi_path, api)
        .config(swagger_config)
}

pub fn api_router(client_id: &str) -> axum::Router<AppState> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/", get(|| async { Redirect::to("/docs") }))
        .nest("/users", users::router())
        .nest("/resources", resources::router())
        .nest("/auth", auth::router())
        .split_for_parts();

    router
        .merge(swagger_ui(api, client_id))
        .layer(TraceLayer::new_for_http())
}
