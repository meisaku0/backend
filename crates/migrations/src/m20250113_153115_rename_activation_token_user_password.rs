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
                    .drop_column(Password::Active)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .rename_column(Password::ActivationToken, Alias::new("password_reset_token"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .add_column(boolean(Password::Active).default(true).not_null())
                    .to_owned(),
            )
            .await?;
        
        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .rename_column(Alias::new("password_reset_token"), Password::ActivationToken)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Password {
    #[sea_orm(iden = "user_password")]
    Table,
    Active,
    ActivationToken,
}
