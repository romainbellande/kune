use super::service;
use crate::graphql::types::{group::Group, user::User};
use crate::modules::auth::guard::AuthGuard;
use crate::modules::casbin::{Permission, RbacGuard, Resource};
use crate::modules::user::service::get_current_user;
use crate::State;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct GroupQuery;

#[Object]
impl GroupQuery {
    #[graphql(guard = "AuthGuard::new()")]
    pub async fn find_user_groups(&self, ctx: &Context<'_>) -> Result<Vec<Group>> {
        let state = ctx.data::<State>().unwrap();
        let current_user: User = get_current_user(ctx, &state.db).await?.into();
        current_user.groups(ctx).await
    }

    #[graphql(guard = "AuthGuard::new().and(RbacGuard::new(Resource::Group, Permission::Read))")]
    pub async fn find_group_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Group> {
        let state = ctx.data::<State>().unwrap();
        service::find_by_id(&state.db, id).await
    }
}
