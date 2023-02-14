use reqwest;
use jsonwebtoken::jwk::JwkSet;
use crate::CONFIG;
use async_graphql::Result;

pub async fn fetch_jwks() -> Result<JwkSet> {
  let uri = format!("http://{}/.well-known/jwks.json", CONFIG.auth0_domain.clone());
  let response = reqwest::get(uri)
    .await?
    .json::<JwkSet>()
    .await?;

  Ok(response)
}
