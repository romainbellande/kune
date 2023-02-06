use async_graphql::{Context, Object, Result};
use entity::user;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::modules::auth::guard::AuthGuard;

#[derive(Default)]
pub struct Query;
use crate::errors::AppError;

#[Object]
impl Query {
    #[graphql(guard = "AuthGuard::new()")]
    async fn get_user_by_external_id(
        &self,
        ctx: &Context<'_>,
        external_id: String,
    ) -> Result<user::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        user::Entity::find()
            .filter(user::Column::ExternalId.contains(&external_id))
            .one(conn)
            .await?
            .ok_or(AppError::UserNotFound(external_id).into())
    }
}
