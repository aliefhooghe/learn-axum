use super::schemas::Claims;
use crate::AppState;
use axum::{extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{Algorithm, TokenData};
use reqwest::StatusCode;

// Axum claims extractor
pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1 - Extract token
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)
            .inspect_err(|_| tracing::warn!("missing authorization"))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)
            .inspect_err(|_| tracing::warn!("missing token"))?;

        // 2 - Read kid from JWT header
        let header = jsonwebtoken::decode_header(token)
            .inspect_err(|err| tracing::warn!("decode error: {}", err))
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let kid = header
            .kid
            .ok_or(StatusCode::UNAUTHORIZED)
            .inspect_err(|_| tracing::warn!("missing key"))?;

        // 3 - Query key from jwks cache
        let key = {
            let cache = state.jwks_cache.read().await;
            cache
                .get(&kid)
                .ok_or(StatusCode::UNAUTHORIZED)
                .inspect_err(|_| tracing::warn!("jwks cache miss"))?
                .clone()
        };

        // 4 - Validate token
        let mut validation = jsonwebtoken::Validation::new(Algorithm::RS256);
        validation.set_issuer(&[format!(
            "{}/realms/{}",
            state.settings.oauth.issuer_url, state.settings.oauth.realm
        )]);
        validation.set_audience(&["account"]);
        let token_data: TokenData<Claims> = jsonwebtoken::decode(token, &key, &validation)
            .inspect_err(|err| tracing::warn!("token decode error: {err}"))
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser(token_data.claims))
    }
}
