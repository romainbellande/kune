use async_graphql::{EmptySubscription, MergedObject, Schema};
use sea_orm::DatabaseConnection;
use crate::modules::user;

#[derive(MergedObject, Default)]
pub struct QueryRoot(user::Query);

#[derive(MergedObject, Default)]
pub struct MutationRoot(user::Mutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn get_schema(conn: DatabaseConnection) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(conn)
    .finish()
}
