#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::shield::{ExpectCt, Prefetch, Referrer, Shield, XssFilter};
use rocket::time::{Duration, OffsetDateTime};
use sea_orm_rocket::Database;
use config::database::migrations::run_migrations;
use config::database::pool::Db;
use shared::Fairings;

#[get("/healthcheck")]
fn index() -> &'static str { "Hello, world!" }

#[launch]
fn rocket() -> _ {
    let shield = Shield::default()
        .enable(Referrer::NoReferrer)
        .enable(Prefetch::Off)
        .enable(ExpectCt::Enforce(Duration::days(30)))
        .enable(XssFilter::EnableBlock);

    let cache_control = Fairings::CacheControl::new()
        .max_age(Duration::hours(1))
        .public()
        .expires(OffsetDateTime::now_utc() + Duration::days(1));

    rocket::build()
        .mount("/", routes![index])
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .attach(Fairings::Helmet)
        .attach(shield)
        .attach(Fairings::RequestId)
        .attach(cache_control)
        .attach(Fairings::Compression)
        .attach(Fairings::Cors::new())
}