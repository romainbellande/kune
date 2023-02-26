use std::sync::{Arc, Mutex};

use casbin::Enforcer;

use crate::prisma::PrismaClient;

#[derive(Clone)]
pub struct State {
    pub db: Arc<PrismaClient>,
    pub enforcer: Arc<Mutex<Enforcer>>,
}

// impl Clone for State {
//   fn clone(&self) -> Self {
//       Self {
//         db: self.db.clone(),
//         enforcer: self.enforcer.clone(),
//       }
//   }
// }
