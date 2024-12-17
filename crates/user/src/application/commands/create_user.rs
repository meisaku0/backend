use rocket::post;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use rocket_validation::Validated;

use crate::presentation::dto::CreateUserDTO;

/// # Create user
///
/// This endpoint is used to create a new user and return the user's
/// information.
#[openapi]
#[post("/", data = "<_user>")]
pub fn action(_user: Validated<Json<CreateUserDTO>>) -> &'static str { "Ok" }