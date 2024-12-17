pub use sea_orm_migration::prelude::*;

mod m20241217_163035_user;
mod m20241217_163741_user_email;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241217_163035_user::Migration),
            Box::new(m20241217_163741_user_email::Migration),
        ]
    }
}
