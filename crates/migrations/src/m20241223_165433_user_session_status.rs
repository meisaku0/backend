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
                    .table(UserSession::Table)
                    .add_column(boolean(UserSession::Active).default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserSession::Table)
                    .drop_column(UserSession::Active)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserSession {
    #[sea_orm(iden = "user_session")]
    Table,
    Active,
}
