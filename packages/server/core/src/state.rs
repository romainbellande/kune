use std::sync::Arc;

use tokio::sync::RwLock;
use casbin::Enforcer;

use crate::prisma::PrismaClient;

#[derive(Clone)]
pub struct State {
    pub db: Arc<PrismaClient>,
    pub enforcer: Arc<RwLock<Enforcer>>,
}
