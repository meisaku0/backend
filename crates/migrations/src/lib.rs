pub use sea_orm_migration::prelude::*;

mod m20241217_163035_user;
mod m20241217_163741_user_email;
mod m20241217_163944_user_password;
mod m20241217_164119_user_email_password_relation;
mod m20241221_161746_user_ban_status;
mod m20241221_174045_user_session;
mod m20241223_150617_user_session_device;
mod m20241223_165433_user_session_status;
mod m20250104_155156_user_avatar;
mod m20250106_223315_remove_url_from_avatar_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241217_163035_user::Migration),
            Box::new(m20241217_163741_user_email::Migration),
            Box::new(m20241217_163944_user_password::Migration),
            Box::new(m20241217_164119_user_email_password_relation::Migration),
            Box::new(m20241221_161746_user_ban_status::Migration),
            Box::new(m20241221_174045_user_session::Migration),
            Box::new(m20241223_150617_user_session_device::Migration),
            Box::new(m20241223_165433_user_session_status::Migration),
            Box::new(m20250104_155156_user_avatar::Migration),
            Box::new(m20250106_223315_remove_url_from_avatar_user::Migration),
        ]
    }
}
