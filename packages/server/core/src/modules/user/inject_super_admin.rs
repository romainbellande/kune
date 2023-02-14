use crate::{CONFIG, prisma::PrismaClient};
use super::service;
use async_graphql::Result;
use crate::graphql::types::user::User;

pub async fn inject_super_admin(prisma_client: &PrismaClient) -> Result<User> {
    service::create_if_not_exists(prisma_client, CONFIG.super_admin.clone())
        .await
        .map(|data| data.into())
}
