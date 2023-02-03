use axum::Server;
use std::net::SocketAddr;

mod state;
pub use state::State;

mod router;
mod ws;
use router::router;

mod config;
use config::CONFIG;

mod modules;

pub async fn start() {
    let app = router();

    let addr = SocketAddr::from(([127, 0, 0, 1], CONFIG.port));

    println!("server listening on http://127.0.0.1:{}", CONFIG.port);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
