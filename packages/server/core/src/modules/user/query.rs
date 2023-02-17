use super::service;
use crate::graphql::types::user::User;
use crate::modules::auth::guard::AuthGuard;
use crate::prisma::PrismaClient;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "AuthGuard::new()")]
    pub async fn get_user_by_external_id(
        &self,
        ctx: &Context<'_>,
        external_id: String,
    ) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();
        service::get_user_by_external_id(db, external_id)
            .await
            .map(|data| data.into())
    }
}
