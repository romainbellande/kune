mod guard;
pub use guard::RbacGuard;
mod acl;
mod extractor;
use crate::CONFIG;
pub use acl::{Permission, Resource};
use sqlx_adapter::casbin::prelude::*;
use sqlx_adapter::casbin::Result;
use sqlx_adapter::SqlxAdapter;

pub async fn init() -> Result<Enforcer> {
    let m = DefaultModel::from_file("casbin/rbac_model.conf").await?;

    let a = SqlxAdapter::new(CONFIG.database_url.clone(), 8).await?;

    Enforcer::new(m, a).await
}
