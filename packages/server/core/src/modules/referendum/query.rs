use super::service;
use crate::modules::auth::guard::AuthGuard;
use crate::State;
use crate::{
    graphql::types::referendum::Referendum, modules::group::extract_gid, utils::PaginatedResult,
};
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct ReferendumQuery;

#[Object]
impl ReferendumQuery {
    #[graphql(guard = "AuthGuard::new()")]
    pub async fn find_referendums(
        &self,
        ctx: &Context<'_>,
        page: Option<i64>,
        limit: Option<i64>,
    ) -> Result<PaginatedResult<Referendum>> {
        let state = ctx.data::<State>().unwrap();
        let gid = extract_gid(ctx)?;
        service::find(&state.db, gid, page, limit).await
    }
}
