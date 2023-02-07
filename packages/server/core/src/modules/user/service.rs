use entity::user::{self, CreateUserDto};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait};
use crate::errors::AppError;
use async_graphql::Result;

pub async fn get_user_by_external_id(conn: &DatabaseConnection, external_id: String) -> Result<user::Model> {
        user::Entity::find()
            .filter(user::Column::ExternalId.contains(&external_id))
            .one(conn)
            .await?
            .ok_or(AppError::UserNotFound(external_id).into())
}

pub async fn create(conn: &DatabaseConnection, dto: CreateUserDto) -> Result<user::Model> {
    let user_model = user::Model::from(dto);
    let user_active_model: user::ActiveModel = user_model.clone().into();

    user_active_model.save(conn).await
      .map_err(|err| AppError::UserCreationError(err.to_string()).into_graphql_error())
      .map(|_| user_model)
}
