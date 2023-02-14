use crate::CONFIG;
use entity::user;
use sea_orm::DatabaseConnection;
use super::service;
use async_graphql::Result;

pub async fn inject_super_admin(conn: &DatabaseConnection) -> Result<user::Model> {
    service::create_if_not_exists(conn, CONFIG.super_admin.clone()).await
}
