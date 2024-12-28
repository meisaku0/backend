use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, default_expr = "Expr::cust(\"gen_random_uuid()\")")]
    pub id: Uuid,
    pub username: String,
    pub email_id: Option<Uuid>,
    pub password_id: Option<Uuid>,
    #[sea_orm(default = "false")]
    pub ban: bool,
    pub ban_reason: Option<String>,
    pub profile_picture_url: Option<String>,
    pub cover_picture_url: Option<String>,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::email::Entity", from = "Column::EmailId", to = "super::email::Column::Id")]
    Email,
    #[sea_orm(
        belongs_to = "super::password::Entity",
        from = "Column::PasswordId",
        to = "super::password::Column::Id"
    )]
    Password,
    #[sea_orm(has_many = "super::user_session::Entity")]
    Sessions,
}

impl Related<super::email::Entity> for Entity {
    fn to() -> RelationDef { Relation::Email.def() }
}

impl Related<super::password::Entity> for Entity {
    fn to() -> RelationDef { Relation::Password.def() }
}

impl Related<super::user_session::Entity> for Entity {
    fn to() -> RelationDef { Relation::Sessions.def() }
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
