use async_graphql::{Context, Object, Result};
use entity::user;
use sea_orm::DatabaseConnection;
use crate::modules::auth::guard::AuthGuard;
use super::service;

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    #[graphql(guard = "AuthGuard::new()")]
    pub async fn get_user_by_external_id(
        &self,
        ctx: &Context<'_>,
        external_id: String,
    ) -> Result<user::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        service::get_user_by_external_id(conn, external_id).await
    }
}
