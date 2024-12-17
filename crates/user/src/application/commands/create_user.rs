use rocket::post;
use rocket::serde::json::Json;
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
pub fn action(user: Validated<Json<CreateUserDTO>>) -> Result<Json<UserCreatedDTO>, Error> { 
    let user = user.into_inner();
    
    Ok(Json(UserCreatedDTO {
        id: 1,
        username: user.username.clone(),
        email: user.email.clone(),
    }))
}
