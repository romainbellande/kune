use crate::{
    errors::AppError,
    modules::auth::token::{AccessToken, AccessTokenRaw},
};
use async_graphql::{Context, Result};

pub async fn extract_uid(ctx: &Context<'_>) -> Result<String> {
    let access_token_string = ctx
        .data_opt::<AccessTokenRaw>()
        .ok_or(AppError::MissingAccessToken.into_graphql_error())?;

    let access_token = AccessToken::from_string(access_token_string.to_string()).await?;

    Ok(access_token.sub)
}
