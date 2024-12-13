use migration::MigratorTrait;
use rocket::{fairing, Build, Rocket};
use sea_orm_rocket::Database;

use crate::database::pool::Db;

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}
