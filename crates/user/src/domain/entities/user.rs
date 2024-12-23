use sea_orm::entity::prelude::*;

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
