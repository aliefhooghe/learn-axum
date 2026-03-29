use crate::AppState;

use super::schemas::JwksResponse;
use jsonwebtoken::DecodingKey;
use std::{collections::HashMap, time::Duration};

pub type JwksCache = HashMap<String, DecodingKey>;

pub async fn fetch_jwks(base_url: &str, realm: &str) -> Result<JwksCache, reqwest::Error> {
    let url = format!(
        "{}/realms/{}/protocol/openid-connect/certs",
        base_url, realm
    );

    tracing::info!("retrieve jwks from {}", &url);
    let response: JwksResponse = reqwest::get(&url).await?.json().await?;

    Ok(response
        .keys
        .into_iter()
        .map(|jwk| {
            let key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e).expect("Invalid JWK");
            (jwk.kid, key)
        })
        .collect())
}

pub async fn refresh_jwks_task(app_state: AppState) {
    let interval_duration = Duration::from_secs(
        app_state
            .settings
            .oauth
            .jwks_cache_refresh_interval_sec
            .unwrap_or(3600),
    );
    let mut interval = tokio::time::interval(interval_duration);

    tracing::info!("start refresh task");

    loop {
        interval.tick().await;

        match fetch_jwks(
            &app_state.settings.oauth.issuer_url,
            &app_state.settings.oauth.realm,
        )
        .await
        {
            Ok(jwks) => {
                let mut cache = app_state.jwks_cache.write().await;
                *cache = jwks;
            }
            Err(err) => tracing::error!("jwks refresh failure: {}", err),
        }
    }
}
