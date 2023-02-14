use async_graphql::{Context, Object, Result};
use crate::modules::auth::guard::AuthGuard;
use super::service;
use crate::prisma::PrismaClient;
use crate::graphql::types::user::User;

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    #[graphql(guard = "AuthGuard::new()")]
    pub async fn get_user_by_external_id(
        &self,
        ctx: &Context<'_>,
        external_id: String,
    ) -> Result<User> {
        let prisma_client = ctx.data::<PrismaClient>().unwrap();
        service::get_user_by_external_id(prisma_client, external_id).await.map(|data| data.into())
    }
}
