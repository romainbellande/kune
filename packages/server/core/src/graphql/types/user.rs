use super::group::Group;

use crate::{errors::AppError, prisma::user, State};
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};

#[derive(InputObject, Clone)]
pub struct CreateUserDto {
    pub name: String,
    pub external_id: String,
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct User {
    pub id: String,
    pub name: String,
    pub external_id: String,
}

#[ComplexObject]
impl User {
    pub async fn groups(&self, ctx: &Context<'_>) -> Result<Vec<Group>> {
        let db = &ctx.data::<State>().unwrap().db;

        let current_user = db
            .user()
            .find_unique(user::id::equals(self.id.clone()))
            .with(user::groups::fetch(vec![]))
            .exec()
            .await?
            .ok_or(AppError::UserNotFound(self.id.clone()).into_graphql_error())?;

        let groups = current_user
            .groups()
            .map_err(|err| AppError::GroupsFindError(err.to_string()).into_graphql_error())?
            .iter()
            .map(|g| ((*g).clone()).into())
            .collect();

        Ok(groups)
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
