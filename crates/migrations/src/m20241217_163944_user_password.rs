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
                    .table(Password::Table)
                    .if_not_exists()
                    .col(pk_uuid(Password::Id))
                    .col(boolean(Password::Active).not_null())
                    .col(string(Password::ActivationToken).not_null())
                    .col(integer(Password::UserId).not_null())
                    .col(
                        date_time(Password::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(Password::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Password::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Password {
    #[sea_orm(iden = "user_password")]
    Table,
    Id,
    Active,
    ActivationToken,
    UserId,
    CreatedAt,
    UpdatedAt,
}
