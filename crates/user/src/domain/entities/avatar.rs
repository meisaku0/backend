use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sea_orm::entity::prelude::*;
use sea_orm::FromQueryResult;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_avatar")]
pub struct Model {
    #[sea_orm(primary_key, default_expr = "Expr::cust(\"gen_random_uuid()\")")]
    pub id: Uuid,
    pub user_id: Uuid,
    pub bucket_name: String,
    pub object_name: String,
    pub location: String,
    pub etag: String,
    pub version_id: Uuid,
    pub variant: Variant,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef { Relation::User.def() }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, DeriveDisplay, Serialize, Deserialize, JsonSchema,
)]
#[serde(crate = "rocket::serde")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_avatar_variant")]
pub enum Variant {
    #[sea_orm(string_value = "original")]
    Original,
    #[sea_orm(string_value = "thumbnail")]
    Thumbnail,
    #[sea_orm(string_value = "small")]
    Small,
    #[sea_orm(string_value = "medium")]
    Medium,
    #[sea_orm(string_value = "large")]
    Large,
}

/// Partial model for `Avatar`
///
/// This is useful for queries that only need a subset of the columns.
#[derive(FromQueryResult, DerivePartialModel, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[sea_orm(entity = "Entity")]
pub struct PartialAvatar {
    /// The variant of the avatar image file
    pub variant: Variant,
    /// The bucket name of the avatar image file in the storage
    pub bucket_name: String,
    /// The object name of the avatar image file in the storage
    pub object_name: String,
}
