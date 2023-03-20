use async_graphql::{Context, Result};
use casbin::{MgmtApi, RbacApi};

use crate::errors::AppError;
use crate::modules::{group, user};
use crate::utils::enum_string;
use std::str::FromStr;

use super::{default::get_default_policies, extractor::extract_enforcer};

enum_string!(Resource, [User, Group, Referendum]);
enum_string!(Permission, [Edit, Read, Write, Delete, Archive]);

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

    // enforcer.write().await.add_role_for_user(user, role, domain)

    let result = enforcer
        .write()
        .await
        .add_policy(vec![uid, gid, resource, permission])
        .await
        .map_err(|err| AppError::AclCreatePolicyError(err.to_string()).into_graphql_error());
    result
}

pub async fn add_default_group_policies(ctx: &Context<'_>, gid: String) -> Result<bool> {
    let default_policies: Vec<Vec<String>> = get_default_policies()
        .into_iter()
        .map(|policies| policies.into_casbin_policies(gid.clone()))
        .flat_map(|item| item.into_iter())
        .collect();

    let enforcer = extract_enforcer(ctx);

    let result = enforcer
        .write()
        .await
        .add_policies(default_policies)
        .await
        .map_err(|err| AppError::AclCreatePolicyError(err.to_string()).into_graphql_error());

    result
}

pub async fn add_user_role(ctx: &Context<'_>, gid: String, role: String) -> Result<bool> {
    let uid = user::extract_uid(ctx).await?;
    let enforcer = extract_enforcer(ctx);

    let result = enforcer
        .write()
        .await
        .add_role_for_user(&uid, &role, Some(&gid))
        .await
        .map_err(|err| AppError::AclCreatePolicyError(err.to_string()).into_graphql_error());

    result
}

pub async fn get_user_roles(ctx: &Context<'_>) -> Result<Vec<String>> {
    let uid = user::extract_uid(ctx).await?;
    let gid = group::extract_gid(ctx)?;
    let enforcer = extract_enforcer(ctx);
    let result = enforcer.write().await.get_roles_for_user(&uid, Some(&gid));
    Ok(result)
}
