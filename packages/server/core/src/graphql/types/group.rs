use super::user::User;
use crate::{
    errors::AppError,
    prisma::{group, user, PrismaClient},
};
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};

#[derive(InputObject, Clone)]
pub struct CreateGroupDto {
    pub name: String,
    pub owner_id: String,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Group {
    id: String,
    name: String,
    owner_id: String,
}

#[ComplexObject]
impl Group {
    pub async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let result = db
            .user()
            .find_many(vec![user::groups::some(vec![group::id::equals(
                self.id.clone(),
            )])])
            .exec()
            .await?
            .into_iter()
            .map(|g| g.into())
            .collect();

        Ok(result)
    }

    pub async fn owner(&self, ctx: &Context<'_>) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();

        let result = db
            .user()
            .find_first(vec![user::owned_groups::some(vec![group::id::equals(
                self.id.clone(),
            )])])
            .exec()
            .await?
            .ok_or(AppError::OwnerNotFound.into_graphql_error())?;

        Ok(result.into())
    }
}

impl From<group::Data> for Group {
    fn from(val: group::Data) -> Self {
        Group {
            id: val.id,
            name: val.name,
            owner_id: val.owner_id,
        }
    }
}
