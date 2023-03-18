use std::sync::Arc;

use casbin::Enforcer;
use tokio::sync::RwLock;

use crate::prisma::PrismaClient;

#[derive(Clone)]
pub struct State {
    pub db: Arc<PrismaClient>,
    pub enforcer: Arc<RwLock<Enforcer>>,
}
