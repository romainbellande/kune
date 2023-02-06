use axum::{response::Result, routing::get, Json, Router};
use axum_macros::debug_handler;

use serde::{Deserialize, Serialize};
use std::convert::Infallible;

pub enum HealthStatus {
    Up,
    Down,
}

impl ToString for HealthStatus {
    fn to_string(&self) -> String {
        let str = match self {
            Self::Up => "up",
            Self::Down => "down",
        };

        str.to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub status: String,
}

impl Health {
    pub fn new() -> Self {
        Self {
            status: HealthStatus::Up.to_string(),
        }
    }
}

#[debug_handler]
pub async fn handler() -> Result<Json<Health>, Infallible> {
    let health = Json(Health::new());
    Ok(health)
}

pub fn router() -> Router {
    Router::new().route("/", get(handler))
}
