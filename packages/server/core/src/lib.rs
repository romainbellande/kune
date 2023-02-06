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

mod db;
mod modules;
use db::Database;

pub async fn start() {
    let conn = Database::new(CONFIG.database_url.clone())
        .get_connection()
        .await;

    let app = router(conn);

    let addr = SocketAddr::from(([127, 0, 0, 1], CONFIG.port));

    println!("server listening on http://127.0.0.1:{}", CONFIG.port);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
