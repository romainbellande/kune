use async_graphql::{Object, Result, Context};
use entity::user::{self, CreateUserDto};
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use crate::errors::AppError;
use crate::modules::auth::guard::AuthGuard;

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
  #[graphql(guard = "AuthGuard::new()")]
  async fn create(&self, ctx: &Context<'_>, create_user_dto: CreateUserDto) -> Result<user::Model> {
    let conn = ctx.data::<DatabaseConnection>().unwrap();
    let user_model = user::Model::from(create_user_dto);
    let user_active_model: user::ActiveModel = user_model.clone().into();

    user_active_model.save(conn).await
      .map_err(|err| AppError::UserCreationError(err.to_string()).into_graphql_error())
      .map(|_| user_model)
  }
}
