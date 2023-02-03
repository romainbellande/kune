use crate::ws;
use crate::State;
use axum::Extension;
use axum::{routing::get, Router};
use crate::modules::healthcheck;

pub fn router() -> Router {
    Router::new()
        .nest("/health", healthcheck::router())
        .route("/ws", get(ws::handler))
        .layer(Extension(State))
}
