use axum::Server;
use std::net::SocketAddr;

mod graphql;
mod state;
pub use state::State;

mod router;
mod ws;
use router::router;

mod config;
mod errors;
use config::CONFIG;

mod modules;

use crate::prisma::PrismaClient;
mod prisma;

pub async fn start() {
    let prisma_client = PrismaClient::_builder().with_url(CONFIG.database_url.clone()).build().await.expect("can't instantiate prisma client");

    prisma_client
        ._db_push()
        .accept_data_loss() // --accept-data-loss in CLI
        .force_reset()
        .await      // --force-reset in CLI
        .expect("could not migrate database");

    modules::user::inject_super_admin(&prisma_client).await
        .expect("an error occurred while injecting super admin");

    let app = router(prisma_client);

    let addr = SocketAddr::from(([127, 0, 0, 1], CONFIG.port));

    println!("server listening on http://127.0.0.1:{}", CONFIG.port);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
