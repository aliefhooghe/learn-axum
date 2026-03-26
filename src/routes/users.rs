use axum::extract::Path;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(Serialize, ToSchema)]
pub struct User {
    id: u32,
    name: String,
}

#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "List users", body = [User])
    )
)]
async fn list_users() -> axum::Json<Vec<User>> {
    axum::Json(vec![
        User {
            id: 1,
            name: "Alice".into(),
        },
        User {
            id: 2,
            name: "Bob".into(),
        },
    ])
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = u32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Single user", body = User)
    )
)]
async fn get_user(Path(id): Path<u32>) -> axum::Json<User> {
    axum::Json(User {
        id,
        name: "Alice".into(),
    })
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_users))
        .routes(routes!(get_user))
}
