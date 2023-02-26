use crate::graphql::types::group::{CreateGroupDto, Group};
use crate::{
    errors::AppError,
    prisma::{group, user, PrismaClient},
    utils::PaginatedResult,
};
use async_graphql::Result;
use prisma_client_rust::or;

pub async fn create(
    db: &PrismaClient,
    owner_id: String,
    dto: CreateGroupDto,
) -> Result<group::Data> {
    db.group()
        .create(
            dto.name,
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
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(20);

    let num_items = db
        .group()
        .count(vec![group::users::some(vec![user::id::equals(
            current_user.id.clone(),
        )])])
        .exec()
        .await
        .map_err(|err| AppError::GroupsCountError(err.to_string()).into_graphql_error())?;

    let data = db
        .group()
        .find_many(vec![or![group::users::some(vec![user::id::equals(
            current_user.id
        )]),]])
        .skip(page * limit)
        .take(limit)
        .exec()
        .await
        .map_err(|err| AppError::GroupsFindError(err.to_string()).into_graphql_error())?;

    let paginated_result = PaginatedResult::<Group> {
        data: data.clone().into_iter().map(|item| item.into()).collect(),
        num_items,
        num_pages: (num_items as f32 / limit as f32).ceil() as i64,
        page,
        page_items: data.len() as i64,
    };

    Ok(paginated_result)
}
