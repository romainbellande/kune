use super::token::{AccessTokenRaw, AccessToken};
use crate::errors::AppError;
use async_graphql::{Context, Error, ErrorExtensions, Guard, Result};
use axum::async_trait;

pub struct AuthGuard;

impl AuthGuard {
    pub fn new() -> Self {
        Self
    }

    pub async fn check_access_token(access_token_string: String) -> Result<(), Error> {
        AccessToken::from_string(access_token_string).await?;

        Ok(())
    }
}

#[async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), Error> {
        let access_token = ctx.data_opt::<AccessTokenRaw>().ok_or({
          AppError::MissingAccessToken.into_graphql_error()
        })?;

        Self::check_access_token(access_token.to_string()).await
    }
}
