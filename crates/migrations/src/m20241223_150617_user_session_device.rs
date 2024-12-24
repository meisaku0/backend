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
                    .add_column(string_null(UserSession::Ip))
                    .add_column(string_null(UserSession::Os))
                    .add_column(string_null(UserSession::Device))
                    .add_column(string_null(UserSession::Browser))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserSession::Table)
                    .drop_column(UserSession::Ip)
                    .drop_column(UserSession::Os)
                    .drop_column(UserSession::Device)
                    .drop_column(UserSession::Browser)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserSession {
    #[sea_orm(iden = "user_session")]
    Table,
    Ip,
    Os,
    Device,
    Browser,
}
