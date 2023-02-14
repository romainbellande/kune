use async_graphql::{SimpleObject, InputObject};
use crate::prisma::user;

#[derive(InputObject, Clone)]
pub struct CreateUserDto {
  pub name: String,
  pub external_id: String,
}

#[derive(SimpleObject)]
pub struct User {
  id: String,
  name: String,
  external_id: String,
}

impl Into<User> for user::Data {
  fn into(self) -> User {
      User {
        id: self.id,
        name: self.name,
        external_id: self.external_id,
      }
  }
}
