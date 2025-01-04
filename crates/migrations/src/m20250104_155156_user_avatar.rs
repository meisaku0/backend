use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
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
                .to_owned()
        ).await
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
