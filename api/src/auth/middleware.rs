use axum::{
    extract::{FromRequestParts, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    extract::CookieJar,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{Algorithm, TokenData};

use crate::{AppState, auth::schemas::Claims};

pub async fn auth_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    // 1 - Extract jwt token
    let token = {
        let Ok(jar) = CookieJar::from_request_parts(&mut parts, &state).await;

        if let Some(token) = jar.get("token").map(|cookie| cookie.value().to_string()) {
            token
        } else {
            TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &state)
                .await
                .map(|bearer| bearer.token().to_string())
                .map_err(|_| StatusCode::UNAUTHORIZED)
                .inspect_err(|_| tracing::warn!("Missing authorization/cookie"))?
        }
    };

    // 2 - Read kid from JWT header
    let header = jsonwebtoken::decode_header(&token)
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

    // 5 - Inject the claims
    let mut req = Request::from_parts(parts, body);
    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
