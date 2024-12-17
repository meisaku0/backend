use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub email_id: i32,
    pub password_id: i32,
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
}

impl Related<super::email::Entity> for Entity {
    fn to() -> RelationDef { Relation::Email.def() }
}

impl Related<super::password::Entity> for Entity {
    fn to() -> RelationDef { Relation::Password.def() }
}

impl ActiveModelBehavior for ActiveModel {}