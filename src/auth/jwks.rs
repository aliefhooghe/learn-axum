use super::schemas::JwksResponse;
use jsonwebtoken::DecodingKey;
use std::collections::HashMap;

pub type JwksCache = HashMap<String, DecodingKey>;

pub async fn fetch_jwks(base_url: &str, realm: &str) -> Result<JwksCache, reqwest::Error> {
    let url = format!(
        "{}/realms/{}/protocol/openid-connect/certs",
        base_url, realm
    );

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
