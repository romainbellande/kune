use async_graphql::{Object, Result, Context};
use crate::modules::auth::guard::AuthGuard;
use super::service;
use crate::graphql::types::user::{User, CreateUserDto};
use crate::PrismaClient;

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
  #[graphql(guard = "AuthGuard::new()")]
  async fn create(&self, ctx: &Context<'_>, dto: CreateUserDto) -> Result<User> {
    let prisma_client = ctx.data::<PrismaClient>().unwrap();
    service::create(prisma_client, dto).await.map(|data| data.into())
  }

  async fn create_user_if_not_exists(&self, ctx: &Context<'_>, dto: CreateUserDto) -> Result<User> {
    let prisma_client = ctx.data::<PrismaClient>().unwrap();
    service::create_if_not_exists(prisma_client, dto).await.map(|data| data.into())
  }
}
