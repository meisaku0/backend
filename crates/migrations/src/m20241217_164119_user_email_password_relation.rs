use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let fk_user_email = TableForeignKey::new()
            .name("fk_user_email")
            .from_tbl(Email::Table)
            .from_col(Email::UserId)
            .to_tbl(User::Table)
            .to_col(User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let fk_user_password = TableForeignKey::new()
            .name("fk_user_password")
            .from_tbl(Password::Table)
            .from_col(Password::UserId)
            .to_tbl(User::Table)
            .to_col(User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
                    .add_foreign_key(&fk_user_email)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Password::Table)
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
    Id,
}

#[derive(DeriveIden)]
enum Email {
    #[sea_orm(iden = "user_email")]
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum Password {
    #[sea_orm(iden = "user_password")]
    Table,
    UserId,
}
