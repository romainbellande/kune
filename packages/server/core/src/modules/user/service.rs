use super::extract_uid;
use crate::graphql::types::user::CreateUserDto;
use crate::{
    errors::AppError,
    prisma::{user, PrismaClient},
};
use async_graphql::{Context, Result};

pub async fn get_user_by_external_id(db: &PrismaClient, external_id: String) -> Result<user::Data> {
    db.user()
        .find_unique(user::external_id::equals(external_id.clone()))
        .exec()
        .await?
        .ok_or(AppError::UserNotFound(external_id).into())
}

pub async fn get_current_user(ctx: &Context<'_>, db: &PrismaClient) -> Result<user::Data> {
    let external_uid = extract_uid(ctx).await?;
    get_user_by_external_id(db, external_uid).await
}

pub async fn create(db: &PrismaClient, dto: CreateUserDto) -> Result<user::Data> {
    db.user()
        .create(dto.name, dto.external_id, vec![])
        .exec()
        .await
        .map_err(|err| AppError::UserCreationError(err.to_string()).into_graphql_error())
}

pub async fn create_if_not_exists(db: &PrismaClient, dto: CreateUserDto) -> Result<user::Data> {
    let result = get_user_by_external_id(db, dto.external_id.clone()).await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => create(db, dto).await,
    }
}
