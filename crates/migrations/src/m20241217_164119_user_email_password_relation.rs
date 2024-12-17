use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let fk_user_email = TableForeignKey::new()
            .name("fk_user_email")
            .from_tbl(User::Table)
            .from_col(User::EmailId)
            .to_tbl(Email::Table)
            .to_col(Email::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let fk_user_password = TableForeignKey::new()
            .name("fk_user_password")
            .from_tbl(User::Table)
            .from_col(User::PasswordId)
            .to_tbl(Password::Table)
            .to_col(Password::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_foreign_key(&fk_user_email)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_foreign_key(&fk_user_password)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Email::Table)
                    .drop_foreign_key(Alias::new("fk_user_email"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .drop_foreign_key(Alias::new("fk_user_password"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "user")]
    Table,
    #[sea_orm(iden = "email_id")]
    EmailId,
    #[sea_orm(iden = "password_id")]
    PasswordId,
}

#[derive(DeriveIden)]
enum Email {
    #[sea_orm(iden = "user_email")]
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Password {
    #[sea_orm(iden = "user_password")]
    Table,
    Id,
}
