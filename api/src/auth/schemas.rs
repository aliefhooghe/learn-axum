use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub n: String,
    pub e: String,
}

#[derive(Deserialize)]
pub struct JwksResponse {
    pub keys: Vec<Jwk>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String,
    pub email: Option<String>,
    pub preferred_username: Option<String>,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuth2CallbackParams {
    pub state: String,
    pub session_state: Option<String>,
    pub iss: String,
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuth2AuthorizationCodeParams<'a> {
    pub client_id: &'a str,
    pub code: &'a str,
    pub redirect_uri: &'a str,
    #[serde(default = "code_grant_type")]
    pub grant_type: &'a str,
}

const AUTHORIZATION_CODE_GRANT_TYPE: &'static str = "authorization_code";

fn code_grant_type() -> &'static str {
    AUTHORIZATION_CODE_GRANT_TYPE
}

impl<'a> OAuth2AuthorizationCodeParams<'a> {
    pub fn new(client_id: &'a str, code: &'a str, redirect_uri: &'a str) -> Self {
        Self {
            client_id: client_id,
            code: code,
            redirect_uri: redirect_uri,
            grant_type: AUTHORIZATION_CODE_GRANT_TYPE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuth2Token {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_expires_in: u64,
    pub refresh_token: String,
    pub token_type: String,
    #[serde(alias = "not-before-policy")]
    pub not_before_policy: u64,
    pub session_state: String,
    pub scope: String,
}

#[derive(Serialize)]
pub struct AuthQuery<'a> {
    pub client_id: &'a str,
    pub redirect_uri: &'a str,
    pub response_type: &'a str,
    pub scope: &'a str,
    pub state: &'a str,
}

#[derive(Deserialize)]
pub struct LoginQuery {
    pub redirect: String,
}
