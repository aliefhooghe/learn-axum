use crate::AppState;
use crate::entities::user;
use crate::schemas::user::User;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use sea_orm::EntityTrait;
use utoipa_axum::{router::OpenApiRouter, routes};

#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "List users", body = [User])
    )
)]
async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = user::Entity::find()
        .all(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = users.into_iter().map(User::from).collect();
    Ok(axum::Json(response))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Single user", body = User)
    )
)]
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<User>, StatusCode> {
    let user = user::Entity::find_by_id(id)
        .one(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(axum::Json(User::from(user)))
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_users))
        .routes(routes!(get_user))
}
