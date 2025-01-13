use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_password")]
pub struct Model {
    #[sea_orm(primary_key, default_expr = "Expr::cust(\"gen_random_uuid()\")")]
    pub id: Uuid,
    pub reset_token: Option<Uuid>,
    pub user_id: Uuid,
    pub hash: String,
    pub salt: String,
    pub reset_token_retry: Option<DateTimeWithTimeZone>,
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
