use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .add_column(date_time_null(Password::ResetTokenRetry).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .drop_column(Password::ResetTokenRetry)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Password {
    #[sea_orm(iden = "user_password")]
    Table,
    ResetTokenRetry,
}
