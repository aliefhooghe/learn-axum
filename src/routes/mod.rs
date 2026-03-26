use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub mod resources;
pub mod users;

#[derive(OpenApi)]
pub struct ApiDoc;

pub fn api_router() -> axum::Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/users", users::router())
        .nest("/resources", resources::router())
        .split_for_parts();

    router.merge(SwaggerUi::new("/docs").url("/docs/openapi.json", api))
}
