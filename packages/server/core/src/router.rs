use crate::graphql;
use crate::modules::healthcheck;
use crate::prisma::PrismaClient;
use crate::ws;
use crate::State;
use axum::Extension;
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

pub fn router(db: PrismaClient) -> Router {
    Router::new()
        .nest("/health", healthcheck::router())
        .nest("/graphql", graphql::router(db))
        .route("/ws", get(ws::handler))
        .layer(Extension(State))
        .layer(CorsLayer::permissive())
}
