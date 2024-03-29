use crate::graphql;
use crate::modules::healthcheck;

use crate::ws;
use crate::State;
use axum::Extension;
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

pub fn router(state: State) -> Router {
    Router::new()
        .nest("/health", healthcheck::router())
        .nest("/graphql", graphql::router(state.clone()))
        .route("/ws", get(ws::handler))
        .layer(Extension(state))
        .layer(CorsLayer::permissive())
}
