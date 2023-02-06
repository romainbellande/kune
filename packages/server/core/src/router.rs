use crate::graphql;
use crate::modules::healthcheck;
use crate::ws;
use crate::State;
use axum::Extension;
use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use tower_http::cors::CorsLayer;

pub fn router(conn: DatabaseConnection) -> Router {
    Router::new()
        .nest("/health", healthcheck::router())
        .nest("/graphql", graphql::router(conn))
        .route("/ws", get(ws::handler))
        .layer(Extension(State))
        .layer(CorsLayer::permissive())
}
