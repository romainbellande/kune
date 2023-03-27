use crate::graphql::types::group::{CreateGroupDto, Group};
use crate::{
    errors::AppError,
    prisma::{group, user, PrismaClient},
    utils::{paginate, PaginatedResult},
};
use async_graphql::Result;

pub async fn create(
    db: &PrismaClient,
    owner_id: String,
    dto: CreateGroupDto,
) -> Result<group::Data> {
    db.group()
        .create(
            dto.name,
            dto.slug,
            dto.private,
            vec![group::users::connect(vec![user::id::equals(owner_id)])],
        )
        .exec()
        .await
        .map_err(|err| AppError::GroupCreationError(err.to_string()).into_graphql_error())
}

pub async fn find_all_user_groups(
    db: &PrismaClient,
    current_user: user::Data,
    page: Option<i64>,
    limit: Option<i64>,
) -> Result<PaginatedResult<Group>> {
    let where_condition = vec![group::users::some(vec![user::id::equals(
        current_user.id.clone(),
    )])];

    paginate!(Group, db.group(), where_condition, page, limit).await
}

pub async fn find_by_id(db: &PrismaClient, id: String) -> Result<Group> {
    let result = db
        .group()
        .find_unique(group::id::equals(id.clone()))
        .exec()
        .await
        .map_err(|err| AppError::GroupsFindError(err.to_string()).into_graphql_error())?
        .ok_or(AppError::GroupNotFound(id))?;

    Ok(result.into())
}
