mod guard;
pub use guard::RbacGuard;
mod acl;
mod default;
mod extractor;
use crate::CONFIG;
pub use acl::{
    add_default_group_policies, add_policy, add_policy_with_gid, add_user_role, Permission,
    Resource,
};
pub use default::DefaultRole;
use sqlx_adapter::casbin::prelude::*;
use sqlx_adapter::casbin::Result;
use sqlx_adapter::SqlxAdapter;

pub async fn init() -> Result<Enforcer> {
    let m = DefaultModel::from_file("casbin/rbac_model.conf").await?;

    let a = SqlxAdapter::new(CONFIG.database_url.clone(), 8).await?;

    Enforcer::new(m, a).await
}
