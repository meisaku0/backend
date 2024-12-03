#[macro_use] extern crate rocket;

use rocket::shield::{ExpectCt, NoSniff, Prefetch, Referrer, Shield, XssFilter};
use rocket::time::Duration;

#[get("/healthcheck")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let report_uri = uri!("https://report.meisaku.app");
    let shield = Shield::default()
        .enable(Referrer::NoReferrer)
        .enable(Prefetch::Off)
        .enable(ExpectCt::ReportAndEnforce(Duration::days(30), report_uri))
        .enable(XssFilter::EnableBlock);

    rocket::build().mount("/", routes![index])
}
