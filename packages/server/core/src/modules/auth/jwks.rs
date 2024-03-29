use crate::CONFIG;
use async_graphql::Result;
use jsonwebtoken::jwk::JwkSet;
use reqwest;

pub async fn fetch_jwks() -> Result<JwkSet> {
    let uri = format!(
        "http://{}/.well-known/jwks.json",
        CONFIG.auth0_domain.clone()
    );
    let response = reqwest::get(uri).await?.json::<JwkSet>().await?;

    Ok(response)
}
