#[macro_use]
extern crate rocket;

use rocket::shield::{ExpectCt, Prefetch, Referrer, Shield, XssFilter};
use rocket::time::Duration;
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

    rocket::build()
        .mount("/", routes![index])
        .attach(Fairings::Helmet)
        .attach(shield)
        .attach(Fairings::RequestId)
}
