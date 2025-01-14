use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserAvatar::Table)
                    .drop_column(UserAvatar::Url)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> { Ok(()) }
}

#[derive(DeriveIden)]
enum UserAvatar {
    #[sea_orm(iden = "user_avatar")]
    Table,
    Url,
}
