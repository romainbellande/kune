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

  async fn create_user_if_not_exists(&self, ctx: &Context<'_>, external_id: String, dto: CreateUserDto) -> Result<user::Model> {
    let conn = ctx.data::<DatabaseConnection>().unwrap();

    let result = service::get_user_by_external_id(conn, external_id).await;

    match result {
      Ok(user) => Ok(user),
      Err(_) =>  {
        service::create(conn, dto).await
      }
    }
  }
}
