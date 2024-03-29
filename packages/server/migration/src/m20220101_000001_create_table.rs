use entity::user::{Column, Entity as User};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let user_table = Table::create()
            .table(User)
            .col(ColumnDef::new(Column::Id).uuid().not_null().primary_key())
            .col(ColumnDef::new(Column::ExternalId).string().not_null())
            .col(ColumnDef::new(Column::Name).string().not_null())
            .to_owned();

        manager.create_table(user_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User).to_owned())
            .await
    }
}
