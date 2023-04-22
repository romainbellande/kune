use async_graphql::{Object, Result, Context};

use super::service;
use crate::{graphql::types::referendum::{CreateReferendumDto, Referendum}, modules::group::extract_gid, State};
use crate::modules::auth::guard::AuthGuard;

#[derive(Default)]
pub struct ReferendumMutation;

#[Object]
impl ReferendumMutation {
    #[graphql(guard = "AuthGuard::new()")]
    async fn create_referendum(&self, ctx: &Context<'_>, dto: CreateReferendumDto) -> Result<Referendum> {
        let state = ctx.data::<State>().unwrap();
        let gid = extract_gid(ctx)?;
        let referendum = service::create(&state.db, gid, dto).await?;
        Ok(referendum)
    }
}

