use super::{extract_uid, service};
use crate::modules::auth::guard::AuthGuard;
use crate::{graphql::types::user::User, State};
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
        let state = ctx.data::<State>().unwrap();
        service::get_user_by_external_id(&state.db, external_id)
            .await
            .map(|data| data.into())
    }

    #[graphql(guard = "AuthGuard::new()")]
    pub async fn get_current_user(&self, ctx: &Context<'_>) -> Result<User> {
        let external_uid = extract_uid(ctx).await?;

        let state = ctx.data::<State>().unwrap();
        service::get_user_by_external_id(&state.db, external_uid)
            .await
            .map(|data| data.into())
    }
}
