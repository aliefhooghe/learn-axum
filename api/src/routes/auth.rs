use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    auth::{middleware::AuthUser, schemas::Claims},
};

#[utoipa::path(
    get,
    path = "/me",
    params(),
    security(
        ("oidc" = ["openid"])
    ),
    responses(
        (status = 200, description = "User information", body = Claims),
        (status = 401, description = "Unauthorized")
    )
)]
async fn get_user_info(AuthUser(claims): AuthUser) -> axum::Json<Claims> {
    axum::Json(claims)
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_user_info))
}
