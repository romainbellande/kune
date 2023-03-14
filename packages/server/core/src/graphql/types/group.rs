use super::user::User;
use crate::{
    prisma::{group, user},
    State,
};
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};

#[derive(InputObject, Clone)]
pub struct CreateGroupDto {
    pub name: String,
    pub private: bool,
    pub slug: String,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub private: bool,
    pub slug: String,
}

#[ComplexObject]
impl Group {
    pub async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let db = &ctx.data::<State>().unwrap().db;

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
}

impl From<group::Data> for Group {
    fn from(val: group::Data) -> Self {
        Group {
            id: val.id,
            name: val.name,
            private: val.private,
            slug: val.slug,
        }
    }
}
