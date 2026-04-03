use super::schemas::Claims;
use crate::AppState;
use axum::{extract::FromRequestParts, http::request::Parts};
use reqwest::StatusCode;

// Axum Auth user extractor: decoupled from claims
pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let claims = parts
            .extensions
            .get::<Claims>()
            .ok_or(StatusCode::UNAUTHORIZED)
            .inspect_err(|_| tracing::warn!("missing claim extension"))?;

        Ok(AuthUser(claims.clone()))
    }
}
