use super::service;
use crate::graphql::types::group::{CreateGroupDto, Group};
use crate::modules::auth::guard::AuthGuard;
use crate::modules::user::extractor::CurrentUser;
use crate::modules::casbin::{add_policy, Resource, Permission};
use crate::{State};
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct GroupMutation;

#[Object]
impl GroupMutation {
    #[graphql(guard = "AuthGuard::new()")]
    async fn create_group(&self, ctx: &Context<'_>, dto: CreateGroupDto) -> Result<Group> {
        let state = ctx.data::<State>().unwrap();
        let current_user = CurrentUser::from_ctx(ctx, &state.db).await?;
        let group = service::create(&state.db, current_user.id, dto)
            .await
            .map(|data| data.into())?;

        add_policy(ctx, Resource::Group, Permission::Admin).await?;

        Ok(group)
    }
}
