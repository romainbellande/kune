use crate::graphql::types::group::CreateGroupDto;
use crate::{
    errors::AppError,
    prisma::{group, PrismaClient, user},
};
use async_graphql::Result;

pub async fn create(db: &PrismaClient, dto: CreateGroupDto) -> Result<group::Data> {
  db.group()
      .create(dto.name, user::id::equals(dto.owner_id), vec![])
      .exec()
      .await
      .map_err(|err| AppError::GroupCreationError(err.to_string()).into_graphql_error())
}
