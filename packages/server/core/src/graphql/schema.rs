use crate::{modules::{user::{UserMutation, UserQuery}, group::GroupMutation}, prisma::PrismaClient};
use async_graphql::{EmptySubscription, MergedObject, Schema};

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutation, GroupMutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn get_schema(db: PrismaClient) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db)
    .finish()
}
