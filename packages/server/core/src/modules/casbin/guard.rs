use super::{
    acl::{Permission, Resource},
    extractor::extract_enforcer,
};
use crate::errors::AppError;
use crate::modules::{group, user};
use async_graphql::{Context, Guard, Result};
use axum::async_trait;
use casbin::{CoreApi};

pub struct RbacGuard {
    resource: Resource,
    permission: Permission,
}

impl RbacGuard {
    pub fn new(resource: Resource, permission: Permission) -> Self {
        Self {
            resource,
            permission,
        }
    }
}

#[async_trait]
impl Guard for RbacGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let uid = user::extract_uid(ctx).await?;
        let gid = group::extract_gid(ctx)?;
        let resource = self.resource.to_string();
        let permission = self.permission.to_string();
        let enforcer = extract_enforcer(ctx);

        enforcer
            .read()
            .await
            .enforce((
                uid.clone(),
                gid.clone(),
                resource.clone(),
                permission.clone(),
            ))
            .map_err(|_| AppError::AclError {
                user: uid,
                resource,
                group: gid,
                permission,
            })?;
        Ok(())
    }
}
