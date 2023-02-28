use std::sync::Arc;
use tokio::sync::RwLock;
use crate::State;
use async_graphql::Context;
use casbin::Enforcer;

pub fn extract_enforcer(ctx: &Context<'_>) -> Arc<RwLock<Enforcer>> {
    let state = ctx.data::<State>().expect("enforcer is missing from state");
    state.enforcer.to_owned()
}
