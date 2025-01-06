use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use sea_orm::FromQueryResult;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, default_expr = "Expr::cust(\"gen_random_uuid()\")")]
    pub id: Uuid,
    pub username: String,
    #[sea_orm(default = "false")]
    pub ban: bool,
    pub ban_reason: Option<String>,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::email::Entity", from = "Column::Id", to = "super::email::Column::UserId")]
    Email,
    #[sea_orm(belongs_to = "super::password::Entity", from = "Column::Id", to = "super::password::Column::UserId")]
    Password,
    #[sea_orm(has_many = "super::session::Entity")]
    Sessions,
    #[sea_orm(belongs_to = "super::avatar::Entity", from = "Column::Id", to = "super::avatar::Column::UserId")]
    Avatar,
}

impl Related<super::email::Entity> for Entity {
    fn to() -> RelationDef { Relation::Email.def() }
}

impl Related<super::password::Entity> for Entity {
    fn to() -> RelationDef { Relation::Password.def() }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef { Relation::Sessions.def() }
}

impl Related<super::avatar::Entity> for Entity {
    fn to() -> RelationDef { Relation::Avatar.def() }
}

impl ActiveModelBehavior for ActiveModel {}

/// Partial model for `User`
///
/// This is useful for queries that only need a subset of the columns.
#[derive(FromQueryResult, DerivePartialModel, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[sea_orm(entity = "Entity")]
pub struct PartialUser {
    /// The username of the user
    pub username: String,

    /// The id of the email associated with the user
    pub id: Uuid,

    /// The ban status of the user
    pub ban: bool,

    /// The reason for the ban
    pub ban_reason: Option<String>,

    /// The time the user was created
    pub created_at: DateTimeWithTimeZone,
}

impl From<Model> for PartialUser {
    fn from(user: Model) -> Self {
        Self {
            username: user.username,
            id: user.id,
            ban: user.ban,
            ban_reason: user.ban_reason,
            created_at: user.created_at,
        }
    }
}
