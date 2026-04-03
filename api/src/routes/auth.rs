use axum::extract::State;
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::from_fn_with_state;
use axum::response::Redirect;
use axum::{extract::Query, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, SameSite};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::auth::middleware::auth_middleware;
use crate::auth::schemas::{AuthQuery, LoginQuery};
use crate::{
    AppState,
    auth::schemas::{OAuth2AuthorizationCodeParams, OAuth2CallbackParams, OAuth2Token},
    auth::{extractors::AuthUser, schemas::Claims},
};

#[utoipa::path(
    get,
    path = "/me",
    params(),
    security(
        ("oidc" = ["openid"])
    ),
    responses(
        (status = StatusCode::OK, description = "User information", body = Claims),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized")
    )
)]
async fn get_user_info(AuthUser(claims): AuthUser) -> axum::Json<Claims> {
    axum::Json(claims)
}

#[utoipa::path(get, path = "/login", params(), responses())]
async fn login(
    State(app): State<AppState>,
    Query(q): Query<LoginQuery>,
) -> Result<Redirect, StatusCode> {
    tracing::debug!("oauth2 login");
    let query = AuthQuery {
        client_id: &app.settings.oauth.client_id,
        redirect_uri: &app.settings.oauth.redirect_url,
        response_type: "code",
        scope: "email profile openid",
        state: &q.redirect,
    };

    let query =
        serde_urlencoded::to_string(&query).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let authorize_url = format!(
        "{}/realms/{}/protocol/openid-connect/auth?{}",
        &app.settings.oauth.issuer_url, &app.settings.oauth.realm, &query
    );

    Ok(Redirect::to(&authorize_url))
}

#[utoipa::path(
    get,
    path = "/callback",
    params(),
    responses(
        (status = StatusCode::SEE_OTHER, description = "Redirect to front"),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized")
    )
)]
async fn callback(
    State(app): State<AppState>,
    Query(params): Query<OAuth2CallbackParams>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::debug!("oauth2 callback: {:?}", params);
    let token_url = format!(
        "{}/realms/{}/protocol/openid-connect/token",
        &app.settings.oauth.issuer_url, &app.settings.oauth.realm
    );
    let token_param = OAuth2AuthorizationCodeParams::new(
        &app.settings.oauth.client_id,
        &params.code,
        &app.settings.oauth.redirect_url,
    );
    let response = app
        .issuer_client
        .post(token_url)
        .form(&token_param)
        .send()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token: OAuth2Token = response
        .json()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let cookie = Cookie::build(("token", &token.access_token))
        .http_only(true)
        .secure(false) // should be true on production https
        .same_site(SameSite::Strict)
        .path("/");

    // Redirect to frontend
    // Note: we are not checking redirect url for dev purpose
    let mut response = Redirect::to(&params.state).into_response();

    // Insert token cookie into response
    response.headers_mut().append(
        axum::http::header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).map_err(|_| StatusCode::UNAUTHORIZED)?,
    );

    Ok(response)
}

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_user_info))
        .layer(from_fn_with_state(state, auth_middleware))
        .routes(routes!(login))
        .routes(routes!(callback))
}
