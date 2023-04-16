use async_graphql::Result;

use crate::{
    errors::AppError,
    graphql::types::referendum::Referendum,
    prisma::{referendum, PrismaClient},
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

pub async fn create(_db: &PrismaClient) {
    todo!();
}
