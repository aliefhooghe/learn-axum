use axum::{response::Redirect, routing::get};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::{Config, SwaggerUi};

use crate::AppState;

mod auth;
mod resources;
mod users;

#[derive(OpenApi)]
pub struct ApiDoc;

pub fn api_router(state: AppState) -> axum::Router<AppState> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route(
            "/",
            get(|| async { Redirect::to("/auth/login?redirect=../docs") }),
        )
        .nest("/users", users::router())
        .nest("/resources", resources::router())
        .nest("/auth", auth::router(state))
        .split_for_parts();

    let swagger_config = Config::default().with_credentials(true);
    let swagger = SwaggerUi::new("/docs")
        .url("/docs/openapi.json", api)
        .config(swagger_config);

    router.merge(swagger).layer(TraceLayer::new_for_http())
}
