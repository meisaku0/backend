use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sea_orm::entity::prelude::*;
use sea_orm::FromQueryResult;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_email")]
pub struct Model {
    #[sea_orm(primary_key, default_expr = "Expr::cust(\"gen_random_uuid()\")")]
    pub id: Uuid,
    pub key: String,
    #[sea_orm(default = "false")]
    pub active: bool,
    pub activation_token: Uuid,
    pub user_id: Uuid,
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

/// Partial model for `UserEmail`
///
/// This is useful for queries that only need a subset of the columns.
#[derive(FromQueryResult, DerivePartialModel, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[sea_orm(entity = "Entity")]
pub struct PartialEmail {
    /// The current user email
    pub key: String,
    /// The date when the current user email was updated
    pub updated_at: DateTimeWithTimeZone,
    /// The current user email is active
    pub active: bool,
}
