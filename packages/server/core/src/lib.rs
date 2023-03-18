use axum::Server;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
mod graphql;
mod state;
pub use state::State;

mod router;
mod ws;
use router::router;

mod config;
mod errors;
use config::CONFIG;
mod utils;

mod modules;

use crate::prisma::PrismaClient;
mod prisma;

pub async fn start() {
    let db = PrismaClient::_builder()
        .with_url(CONFIG.database_url.clone())
        .build()
        .await
        .expect("can't instantiate prisma client");

    db._db_push()
        .accept_data_loss() // --accept-data-loss in CLI
        .force_reset()
        .await // --force-reset in CLI
        .expect("could not migrate database");

    println!("migration finalized");

    let enforcer = modules::casbin::init()
        .await
        .expect("could not initialize casbin");

    let state = State {
        db: Arc::new(db),
        enforcer: Arc::new(RwLock::new(enforcer)),
    };

    let _addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.port));

    modules::user::inject_super_admin(state.db.clone())
        .await
        .expect("an error occurred while injecting super admin");

    let app = router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.port));

    println!("server listening on http://127.0.0.1:{}", CONFIG.port);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
