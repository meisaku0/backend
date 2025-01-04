use sea_orm::entity::prelude::*;

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

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
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
