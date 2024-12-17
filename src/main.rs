#[macro_use]
extern crate rocket;

use config::database::pool::Db;
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::shield::{ExpectCt, Prefetch, Referrer, Shield, XssFilter};
use rocket::time::Duration;
use rocket_okapi::rapidoc::{make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{
    get_nested_endpoints_and_docs, mount_endpoints_and_merged_docs, openapi,
    openapi_get_routes_spec,
};
use schemars::JsonSchema;
use sea_orm_rocket::{Connection, Database};
use shared::responses::error::Error;
use shared::Fairings;

/// # Database status
///
/// Status and connection count of the database.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct HealthDatabase {
    /// The status of the database connection.
    pub status: bool,
    /// The number of idle connections.
    pub connections: usize,
}

/// # Server status
///
/// A response with a status code of 200 means the server is up and running.
/// A response with a status code of 500 means the server is down.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct HealthCheck {
    /// The status of the database.
    pub database: HealthDatabase,
}

/// # Healthcheck
///
/// This endpoint is used to check if the server is up and running.
#[openapi]
#[get("/healthcheck")]
async fn index(conn: Connection<'_, Db>) -> Result<Json<HealthCheck>, Error> {
    let db = conn.into_inner();

    Ok(Json(HealthCheck {
        database: HealthDatabase {
            status: db.ping().await.is_ok(),
            connections: db.get_postgres_connection_pool().num_idle(),
        },
    }))
}

#[launch]
fn rocket() -> _ {
    let shield = Shield::default()
        .enable(Referrer::NoReferrer)
        .enable(Prefetch::Off)
        .enable(ExpectCt::Enforce(Duration::days(30)))
        .enable(XssFilter::EnableBlock);

    let cache_control = Fairings::CacheControl::new().no_cache();

    let mut rocket = rocket::build()
        .mount("/", FileServer::from(rocket::fs::relative!("/assets")))
        .register("/", catchers![rocket_validation::validation_catcher])
        .attach(Db::init())
        .attach(Fairings::Helmet)
        .attach(shield)
        .attach(Fairings::RequestId)
        .attach(cache_control)
        .attach(Fairings::Compression)
        .attach(Fairings::Cors::new())
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("Meisaku API Docs".to_owned()),
                general: rocket_okapi::rapidoc::GeneralConfig {
                    spec_urls: vec![rocket_okapi::settings::UrlObject::new("General", "../openapi.json")],
                    update_route: true,
                    heading_text: "Meisaku API Docs".to_owned(),
                    persist_auth: true,
                    sort_tags: true,
                    sort_endpoints_by: rocket_okapi::rapidoc::SortEndpointsBy::Method,
                    ..Default::default()
                },
                hide_show: rocket_okapi::rapidoc::HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    allow_spec_file_download: false,
                    ..Default::default()
                },
                ui: rocket_okapi::rapidoc::UiConfig {
                    header_color: "#f9afb4".to_owned(),
                    theme: rocket_okapi::rapidoc::Theme::Dark,
                    ..Default::default()
                },
                slots: rocket_okapi::rapidoc::SlotsConfig {
                    logo: Some("/uwu.jpg".to_owned()),
                    ..Default::default()
                },
                ..Default::default()
            }),
        );

    let openapi_settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        rocket, "/".to_owned(), openapi_settings,
        "/" => get_nested_endpoints_and_docs! {
            "/healthcheck" => openapi_get_routes_spec![openapi_settings: index],
            "/user" => user::infrastructure::http::get_routes_and_docs(&openapi_settings),
        },
    }

    rocket
}
