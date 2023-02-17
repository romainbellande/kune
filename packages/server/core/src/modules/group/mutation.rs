use super::service;
use crate::graphql::types::group::{CreateGroupDto, Group};
use crate::modules::auth::guard::AuthGuard;
use crate::PrismaClient;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct GroupMutation;

#[Object]
impl GroupMutation {
    #[graphql(guard = "AuthGuard::new()")]
    async fn create_group(&self, ctx: &Context<'_>, dto: CreateGroupDto) -> Result<Group> {
        let db = ctx.data::<PrismaClient>().unwrap();
        service::create(db, dto).await.map(|data| data.into())
    }
}
