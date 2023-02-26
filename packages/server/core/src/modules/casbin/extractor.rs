use std::sync::{Arc, Mutex};

use crate::State;
use async_graphql::Context;
use casbin::Enforcer;

pub fn extract_enforcer(ctx: &Context<'_>) -> Arc<Mutex<Enforcer>> {
    let state = ctx.data::<State>().expect("enforcer is missing from state");
    state.enforcer.to_owned()
}
