#[macro_use]
extern crate rocket;

use std::path::Path;

use auth::jwt::JwtAuth;
use config::database::pool::Db;
use config::AppConfig;
use email::ResendMailer;
use rocket::figment::providers::Format;
use rocket::fs::{FileServer, NamedFile};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::shield::{ExpectCt, Prefetch, Referrer, Shield, XssFilter};
use rocket::time::Duration;
use rocket_okapi::rapidoc::{make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{get_nested_endpoints_and_docs, mount_endpoints_and_merged_docs, openapi, openapi_get_routes_spec};
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

/// # Favicon
///
/// This endpoint is used to serve the favicon.
#[openapi()]
#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new(rocket::fs::relative!("assets/favicon.ico")))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment();
    let app_config = figment.extract::<AppConfig>().unwrap();

    let resend_mail = ResendMailer::new(
        app_config.resend_api_key.unwrap_or_default(),
        app_config.resend_from_email.unwrap_or_default(),
    );

    let shield = Shield::default()
        .enable(Referrer::NoReferrer)
        .enable(Prefetch::Off)
        .enable(ExpectCt::Enforce(Duration::days(30)))
        .enable(XssFilter::EnableBlock);

    let cache_control = Fairings::CacheControl::new().no_cache();

    let mut rocket = rocket::custom(figment)
        .mount("/assets", FileServer::from(rocket::fs::relative!("/assets")))
        .mount("/", routes![favicon])
        .register("/", catchers![rocket_validation::validation_catcher])
        .manage(JwtAuth::new(app_config.jwt_secret.unwrap_or_default().to_string()))
        .manage(resend_mail)
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
                    logo: Some("/assets/uwu.jpg".to_owned()),
                    ..Default::default()
                },
                ..Default::default()
            }),
        );

    let openapi_settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        rocket, "/".to_owned(), openapi_settings,
        "/" => get_nested_endpoints_and_docs! {
            "/" => openapi_get_routes_spec![openapi_settings: index],
            "/user" => user::infrastructure::http::routes::get_routes_and_docs(&openapi_settings),
        },
    }

    rocket
}
