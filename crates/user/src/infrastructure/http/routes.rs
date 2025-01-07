use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::openapi_get_routes_spec;
use rocket_okapi::settings::OpenApiSettings;

use crate::infrastructure::http::controller;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings:
        controller::create,
        controller::activate,
        controller::sign_in,
        controller::refresh_session,
        controller::me,
        controller::sign_out,
        controller::sessions,
        controller::revoke_session,
        controller::revoke_session_by_id,
        controller::change_password,
        controller::change_avatar,
        controller::change_username
    ]
}
