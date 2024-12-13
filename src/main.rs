#[macro_use]
extern crate rocket;

use config::database::migrations::run_migrations;
use config::database::pool::Db;
use rocket::fairing::AdHoc;
use rocket::shield::{ExpectCt, Prefetch, Referrer, Shield, XssFilter};
use rocket::time::{Duration, OffsetDateTime};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes};
use sea_orm_rocket::Database;
use shared::Fairings;

/// Healthcheck
/// 
/// This endpoint is used to check if the server is up and running.
#[openapi]
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
        .mount("/", openapi_get_routes![index])
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .attach(Fairings::Helmet)
        .attach(shield)
        .attach(Fairings::RequestId)
        .attach(cache_control)
        .attach(Fairings::Compression)
        .attach(Fairings::Cors::new())
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/openapi.json".to_string(),
                ..Default::default()
            }),
        )
}
