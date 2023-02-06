use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    pub external_id: String,

    pub name: String,
}

impl From<CreateUserDto> for Model {
    fn from(value: CreateUserDto) -> Self {
        Self {
            id: Uuid::new_v4(),
            external_id: value.external_id,
            name: value.name,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, InputObject)]
pub struct CreateUserDto {
    pub external_id: String,

    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
