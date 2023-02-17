use super::service;
use crate::graphql::types::user::{CreateUserDto, User};
use crate::modules::auth::guard::AuthGuard;
use crate::PrismaClient;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[graphql(guard = "AuthGuard::new()")]
    async fn create_user(&self, ctx: &Context<'_>, dto: CreateUserDto) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();
        service::create(db, dto).await.map(|data| data.into())
    }

    #[graphql(guard = "AuthGuard::new()")]
    async fn create_user_if_not_exists(
        &self,
        ctx: &Context<'_>,
        dto: CreateUserDto,
    ) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();
        service::create_if_not_exists(db, dto)
            .await
            .map(|data| data.into())
    }
}
