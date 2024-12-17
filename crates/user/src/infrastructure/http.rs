use rocket_okapi::openapi_get_routes;
use crate::application;

pub fn routes() -> Vec<rocket::Route> { openapi_get_routes![application::commands::create_user::action] }