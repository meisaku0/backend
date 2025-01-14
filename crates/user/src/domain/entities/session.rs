use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sea_orm::entity::prelude::*;
use sea_orm::FromQueryResult;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_session")]
pub struct Model {
    #[sea_orm(primary_key, default_expr = "Expr::cust(\"gen_random_uuid()\")")]
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub token_type: TokenType,
    pub ip: String,
    pub os: String,
    pub device: String,
    pub browser: String,
    #[sea_orm(default = "true")]
    pub active: bool,
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

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "session_token_type")]
pub enum TokenType {
    #[sea_orm(string_value = "access")]
    Access,
    #[sea_orm(string_value = "refresh")]
    Refresh,
}

/// This struct is used to represent a session in a minimal form.
#[derive(FromQueryResult, DerivePartialModel, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(crate = "rocket::serde")]
#[sea_orm(entity = "Entity")]
pub struct SessionMinimal {
    /// The ID of the session.
    pub id: Uuid,
    /// The ID of the user.
    pub ip: String,
    /// The operating system of the device.
    pub os: String,
    /// The device of the session.
    pub device: String,
    /// The browser of the session.
    pub browser: String,
    /// The date and time the session was created.
    pub created_at: DateTimeWithTimeZone,
    /// The date and time the session was last updated.
    pub updated_at: DateTimeWithTimeZone,
}
