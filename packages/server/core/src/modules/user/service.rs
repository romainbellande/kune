use crate::{errors::AppError, prisma::{PrismaClient, user}};
use async_graphql::Result;
use crate::graphql::types::user::CreateUserDto;

pub async fn get_user_by_external_id(prisma_client: &PrismaClient, external_id: String) -> Result<user::Data> {
    prisma_client.user().find_unique(user::external_id::equals(external_id.clone()))
      .exec().await?
      .ok_or(AppError::UserNotFound(external_id).into())
}

pub async fn create(prisma_client: &PrismaClient, dto: CreateUserDto) -> Result<user::Data> {
    prisma_client.user().create(dto.name, dto.external_id, vec![]).exec()
      .await
      .map_err(|err| AppError::UserCreationError(err.to_string()).into_graphql_error())
}

pub async fn create_if_not_exists(prisma_client: &PrismaClient, dto: CreateUserDto) -> Result<user::Data> {
  let result = get_user_by_external_id(prisma_client, dto.external_id.clone()).await;

  match result {
    Ok(user) => Ok(user),
    Err(_) =>  {
      create(prisma_client, dto).await
    }
  }
}
