use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

use crate::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub(crate) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TokenType::Enum)
                    .values([TokenType::Refresh, TokenType::Access])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserSession::Table)
                    .col(pk_uuid(UserSession::Id).unique_key())
                    .col(string(UserSession::Token).not_null())
                    .col(string(UserSession::TokenType).custom(TokenType::Enum).not_null())
                    .col(uuid(UserSession::UserId).not_null())
                    .col(
                        date_time(UserSession::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(UserSession::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let fk_user_session = TableForeignKey::new()
            .name("fk_user_session")
            .from_tbl(UserSession::Table)
            .from_col(UserSession::UserId)
            .to_tbl(User::Table)
            .to_col(User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(UserSession::Table)
                    .add_foreign_key(&fk_user_session)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserSession::Table).to_owned())
            .await?;

        manager.drop_type(Type::drop().name(TokenType::Enum).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "user")]
    Table,
    Id,
}

#[derive(DeriveIden)]
enum UserSession {
    #[sea_orm(iden = "user_session")]
    Table,
    Id,
    UserId,
    Token,
    TokenType,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum TokenType {
    #[sea_orm(iden = "session_token_type")]
    Enum,
    Access,
    Refresh,
}
