use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

use crate::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Variant::Enum)
                    .values([
                        Variant::Large,
                        Variant::Medium,
                        Variant::Small,
                        Variant::Thumbnail,
                        Variant::Original,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserAvatar::Table)
                    .col(uuid(UserAvatar::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(uuid(UserAvatar::UserId).not_null())
                    .col(string(UserAvatar::BucketName).not_null())
                    .col(string(UserAvatar::ObjectName).not_null())
                    .col(string(UserAvatar::Location).not_null())
                    .col(string(UserAvatar::Etag).not_null())
                    .col(uuid(UserAvatar::VersionId).not_null())
                    .col(string(UserAvatar::Url).not_null())
                    .col(string(UserAvatar::Variant).custom(Variant::Enum))
                    .col(
                        date_time(UserAvatar::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(UserAvatar::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let fk_user_session = TableForeignKey::new()
            .name("fk_user_session")
            .from_tbl(UserAvatar::Table)
            .from_col(UserAvatar::UserId)
            .to_tbl(User::Table)
            .to_col(User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(UserAvatar::Table)
                    .add_foreign_key(&fk_user_session)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserAvatar::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserAvatar {
    #[sea_orm(iden = "user_avatar")]
    Table,
    Id,
    UserId,
    BucketName,
    ObjectName,
    Location,
    Etag,
    VersionId,
    Url,
    Variant,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Variant {
    #[sea_orm(iden = "user_avatar_variant")]
    Enum,
    Original,
    Thumbnail,
    Small,
    Medium,
    Large,
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "user")]
    Table,
    Id,
}