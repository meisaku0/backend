use rocket::serde::json::Json;
use rocket::{post};
use rocket_okapi::openapi;
use rocket_validation::Validated;
use shared::responses::error::Error;

use crate::presentation::dto::{CreateUserDTO, UserCreatedDTO};

/// # Create user
///
/// This endpoint is used to create a new user and return the user's
/// information.
#[openapi]
#[post("/", data = "<user>")]
pub fn create(user: Validated<Json<CreateUserDTO>>) -> Result<Json<UserCreatedDTO>, Error> {
    crate::application::commands::create_user::action(user)
}
