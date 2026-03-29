use crate::AppState;
use crate::schemas::user::User as UserSchema;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use entity::prelude::User as UserEntity;
use sea_orm::EntityTrait;
use utoipa_axum::{router::OpenApiRouter, routes};

#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "List users", body = [UserSchema])
    )
)]
async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<UserSchema>>, StatusCode> {
    let users = UserEntity::find()
        .all(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = users.into_iter().map(UserSchema::from).collect();
    Ok(axum::Json(response))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Single user", body = UserSchema)
    )
)]
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<UserSchema>, StatusCode> {
    let user = UserEntity::find_by_id(id)
        .one(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(axum::Json(UserSchema::from(user)))
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_users))
        .routes(routes!(get_user))
}
