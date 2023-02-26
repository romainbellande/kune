use super::service::get_user_by_external_id;
use crate::errors::AppError;
use crate::graphql::types::user::User;
use crate::modules::auth::token::{AccessToken, AccessTokenRaw};
use crate::prisma::PrismaClient;
use async_graphql::{Context, Result};

pub struct CurrentUser;

impl CurrentUser {
    pub async fn from_ctx(ctx: &Context<'_>, db: &PrismaClient) -> Result<User> {
        let access_token_raw = ctx
            .data_opt::<AccessTokenRaw>()
            .ok_or(AppError::MissingAccessToken.into_graphql_error())?;
        let access_token = AccessToken::from_string(access_token_raw.to_string()).await?;
        let uid = access_token.sub;
        let current_user: User = get_user_by_external_id(db, uid).await?.into();
        Ok(current_user)
    }
}
