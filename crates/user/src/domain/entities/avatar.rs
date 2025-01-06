use rocket::serde::ser::SerializeStruct;
use rocket::serde::{Deserialize, Serialize, Serializer};
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
    pub url: String,
    pub variant: Variant,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity", from = "Column::UserId", to = "super::user::Column::Id")]
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
#[derive(FromQueryResult, DerivePartialModel, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[sea_orm(entity = "Entity")]
pub struct PartialAvatar {
    /// The URL of the avatar image file in the storage
    pub url: String,
    /// The variant of the avatar image file
    pub variant: Variant,
    /// The bucket name of the avatar image file in the storage
    pub bucket_name: String,
    /// The object name of the avatar image file in the storage
    pub object_name: String,
    /// The location of the avatar image file in the storage
    pub location: String,
}

impl Serialize for PartialAvatar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PartialAvatar", 5)?;

        state.serialize_field("url", format!("{}/avatars/{}", self.bucket_name, self.object_name).as_str())?;
        state.serialize_field("variant", &self.variant)?;
        state.serialize_field("bucket_name", &self.bucket_name)?;
        state.serialize_field("object_name", &self.object_name)?;
        state.serialize_field("location", &self.location)?;
        state.end()
    }
}
