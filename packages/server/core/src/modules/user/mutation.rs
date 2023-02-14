use async_graphql::{Object, Result, Context};
use entity::user::{self, CreateUserDto};
use sea_orm::DatabaseConnection;
use crate::modules::auth::guard::AuthGuard;
use super::service;

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
  #[graphql(guard = "AuthGuard::new()")]
  async fn create(&self, ctx: &Context<'_>, dto: CreateUserDto) -> Result<user::Model> {
    let conn = ctx.data::<DatabaseConnection>().unwrap();
    service::create(conn, dto).await
  }

  async fn create_user_if_not_exists(&self, ctx: &Context<'_>, dto: CreateUserDto) -> Result<user::Model> {
    let conn = ctx.data::<DatabaseConnection>().unwrap();
    service::create_if_not_exists(conn, dto).await
  }
}
