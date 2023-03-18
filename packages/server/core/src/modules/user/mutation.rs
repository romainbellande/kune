use super::service;
use crate::graphql::types::user::{CreateUserDto, User};
use crate::modules::auth::guard::AuthGuard;
use crate::State;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[graphql(guard = "AuthGuard::new()")]
    async fn create_user(&self, ctx: &Context<'_>, dto: CreateUserDto) -> Result<User> {
        let state = ctx.data::<State>().unwrap();
        service::create(&state.db, dto)
            .await
            .map(|data| data.into())
    }

    #[graphql(guard = "AuthGuard::new()")]
    async fn create_user_if_not_exists(
        &self,
        ctx: &Context<'_>,
        dto: CreateUserDto,
    ) -> Result<User> {
        let state = ctx.data::<State>().unwrap();
        service::create_if_not_exists(&state.db, dto)
            .await
            .map(|data| data.into())
    }
}
