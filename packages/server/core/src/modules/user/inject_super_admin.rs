use std::sync::Arc;

use super::service;
use crate::graphql::types::user::User;
use crate::{prisma::PrismaClient, CONFIG};
use async_graphql::Result;

pub async fn inject_super_admin(db: Arc<PrismaClient>) -> Result<User> {
    service::create_if_not_exists(&db, CONFIG.super_admin.clone())
        .await
        .map(|data| data.into())
}
