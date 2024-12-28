use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(User::Table)
                .add_column(string_null(User::ProfilePictureUrl))
                .add_column(string_null(User::CoverPictureUrl))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(User::Table)
                .drop_column(User::ProfilePictureUrl)
                .drop_column(User::CoverPictureUrl)
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "user")]
    Table,
    ProfilePictureUrl,
    CoverPictureUrl,
}
