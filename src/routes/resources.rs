use axum::extract::Path;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(Serialize, ToSchema)]
pub struct Resource {
    id: u32,
    kind: String,
}

#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "List resources", body = [Resource])
    )
)]
async fn list_resources() -> axum::Json<Vec<Resource>> {
    axum::Json(vec![
        Resource {
            id: 1,
            kind: "kind1".into(),
        },
        Resource {
            id: 2,
            kind: "kind2".into(),
        },
    ])
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = u32, Path, description = "Resource ID")
    ),
    responses(
        (status = 200, description = "Single resource", body = Resource)
    )
)]
async fn get_resource(Path(id): Path<u32>) -> axum::Json<Resource> {
    axum::Json(Resource {
        id,
        kind: "kind5".into(),
    })
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_resources))
        .routes(routes!(get_resource))
}
