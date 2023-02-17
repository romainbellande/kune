use super::group::Group;
use crate::prisma::{group, user, PrismaClient};
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};

#[derive(InputObject, Clone)]
pub struct CreateUserDto {
    pub name: String,
    pub external_id: String,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    id: String,
    name: String,
    external_id: String,
}

#[ComplexObject]
impl User {
    pub async fn groups(&self, ctx: &Context<'_>) -> Result<Vec<Group>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let result = db
            .group()
            .find_many(vec![group::users::some(vec![user::id::equals(
                self.id.clone(),
            )])])
            .exec()
            .await?
            .into_iter()
            .map(|g| g.into())
            .collect();

        Ok(result)
    }
}

impl From<user::Data> for User {
    fn from(val: user::Data) -> Self {
        User {
            id: val.id,
            name: val.name,
            external_id: val.external_id,
        }
    }
}
