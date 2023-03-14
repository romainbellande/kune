use async_graphql::{Context, Result};
use casbin::MgmtApi;

use crate::errors::AppError;
use crate::modules::{group, user};
use crate::utils::enum_string;
use std::str::FromStr;

use super::extractor::extract_enforcer;

enum_string!(Resource, [User, Group]);

enum_string!(Permission, [Admin, Read, Write]);

pub async fn add_policy(
    ctx: &Context<'_>,
    resource: Resource,
    permission: Permission,
) -> Result<bool> {
    let gid = group::extract_gid(ctx)?;
    add_policy_with_gid(ctx, gid, resource, permission).await
}

pub async fn add_policy_with_gid(
    ctx: &Context<'_>,
    gid: String,
    resource: Resource,
    permission: Permission,
) -> Result<bool> {
    let uid = user::extract_uid(ctx).await?;
    let resource = resource.to_string();
    let permission = permission.to_string();
    let enforcer = extract_enforcer(ctx);

    let result = enforcer
        .write()
        .await
        .add_policy(vec![uid, gid, resource, permission])
        .await
        .map_err(|err| AppError::AclCreatePolicyError(err.to_string()).into_graphql_error());
    result
}
