use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;

pub mod auth;
pub mod resources;
pub mod users;

#[derive(OpenApi)]
pub struct ApiDoc;

pub fn api_router() -> axum::Router<AppState> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/users", users::router())
        .nest("/resources", resources::router())
        .nest("/auth", auth::router())
        .split_for_parts();

    router.merge(SwaggerUi::new("/docs").url("/docs/openapi.json", api))
}
