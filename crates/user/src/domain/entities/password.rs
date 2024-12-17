use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_password")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub active: bool,
    pub activation_token: Uuid,
    pub user_id: Uuid,
    pub hash: String,
    pub salt: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity", from = "Column::UserId", to = "super::user::Column::Id")]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef { crate::domain::entities::email::Relation::User.def() }
}

impl ActiveModelBehavior for ActiveModel {}
