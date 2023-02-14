use jsonwebtoken::jwk::Jwk;
use super::jwks::fetch_jwks;
use async_graphql::Result;
use crate::errors::AppError;

pub async fn get_jwk(kid: String) -> Result<Jwk> {
    let jwks = fetch_jwks().await?;

    jwks.find(&kid)
        .ok_or(AppError::MissingJwk.into_graphql_error()).cloned()
}
