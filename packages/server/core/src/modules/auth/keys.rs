use super::jwks::fetch_jwks;
use crate::errors::AppError;
use async_graphql::Result;
use jsonwebtoken::jwk::Jwk;

pub async fn get_jwk(kid: String) -> Result<Jwk> {
    let jwks = fetch_jwks().await?;

    jwks.find(&kid)
        .ok_or(AppError::MissingJwk.into_graphql_error())
        .cloned()
}
