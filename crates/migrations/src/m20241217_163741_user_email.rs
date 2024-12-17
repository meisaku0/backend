use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Email::Table)
                    .if_not_exists()
                    .col(pk_uuid(Email::Id).integer())
                    .col(boolean(Email::Active).not_null())
                    .col(string(Email::ActivationToken).not_null())
                    .col(integer(Email::UserId).not_null())
                    .col(
                        date_time(Email::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(Email::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Email::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Email {
    #[sea_orm(iden = "user_email")]
    Table,
    Id,
    Active,
    ActivationToken,
    UserId,
    CreatedAt,
    UpdatedAt,
}
