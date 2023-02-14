use async_graphql::{EmptySubscription, MergedObject, Schema};
use crate::{modules::user, prisma::PrismaClient};

#[derive(MergedObject, Default)]
pub struct QueryRoot(user::Query);

#[derive(MergedObject, Default)]
pub struct MutationRoot(user::Mutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn get_schema(prisma_client: PrismaClient) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(prisma_client)
    .finish()
}
