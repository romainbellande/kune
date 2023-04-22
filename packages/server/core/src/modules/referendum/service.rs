use std::str::FromStr;

use async_graphql::Result;
use prisma_client_rust::chrono::{DateTime, FixedOffset};

use crate::{
    errors::AppError,
    graphql::types::referendum::{CreateReferendumDto, Referendum},
    prisma::{group, referendum, PrismaClient},
    utils::{paginate, PaginatedResult},
};

pub async fn find(
    db: &PrismaClient,
    gid: String,
    page: Option<i64>,
    limit: Option<i64>,
) -> Result<PaginatedResult<Referendum>> {
    let where_condition = vec![referendum::group_id::equals(gid)];

    paginate!(Referendum, db.referendum(), where_condition, page, limit).await
}

pub async fn update(_db: &PrismaClient) {
    todo!();
}

pub async fn find_by_id(_db: &PrismaClient) {
    todo!();
}

pub async fn create(
    db: &PrismaClient,
    gid: String,
    dto: CreateReferendumDto,
) -> Result<Referendum> {
    let end_date = DateTime::<FixedOffset>::from_str(&(dto.end_date))
        .map_err(|err| AppError::InvalidDate(err.to_string()).into_graphql_error())?;

    let data = db
        .referendum()
        .create(
            group::id::equals(gid),
            dto.name,
            dto.slug,
            dto.question,
            dto.participants.into(),
            end_date,
            vec![],
        )
        .exec()
        .await
        .map_err(|err| AppError::DbError(err.to_string()).into_graphql_error())?;

    Ok(data.into())
}
