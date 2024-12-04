#[macro_use]
extern crate rocket;

use rocket::shield::{ExpectCt, Prefetch, Referrer, Shield, XssFilter};
use rocket::time::{Duration, OffsetDateTime};
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
        .attach(Fairings::Helmet)
        .attach(shield)
        .attach(Fairings::RequestId)
        .attach(cache_control)
        .attach(Fairings::Compression)
}
