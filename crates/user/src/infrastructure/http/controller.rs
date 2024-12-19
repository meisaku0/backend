use config::database::pool::Db;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{patch, post};
use rocket_okapi::openapi;
use rocket_validation::Validated;
use sea_orm_rocket::Connection;
use shared::responses::error::Error;

use crate::presentation::dto::active_email::ActiveEmailDTO;
use crate::presentation::dto::create_user::{CreateUserDTO, UserCreatedDTO};
use crate::presentation::dto::sign_in::CredentialsDTO;

/// # Create
///
/// This endpoint is used to create a new user and return the user's
/// information.
#[openapi(ignore = "conn", tag = "User")]
#[post("/", data = "<user>")]
pub async fn create(
    user: Validated<Json<CreateUserDTO>>, conn: Connection<'_, Db>,
) -> Result<Json<UserCreatedDTO>, Error> {
    crate::application::commands::create_user::action(user.into_deep_inner(), conn.into_inner()).await
}

/// # Activate
///
/// This endpoint is used to activate a user's email. The user must provide
/// the activation code sent to their email. If the activation code is valid,
/// the user's email will be activated. If the activation code is invalid, an
/// error will be returned.
#[openapi(ignore = "conn", tag = "User")]
#[patch("/activate", data = "<activation>")]
pub async fn activate(activation: Validated<Json<ActiveEmailDTO>>, conn: Connection<'_, Db>) -> Result<Status, Error> {
    crate::application::commands::activate_email::action(activation.into_deep_inner(), conn.into_inner()).await
}

/// # Sign In
/// 
/// This endpoint is used to sign in a user. The user must provide their email
/// and password. If the email and password are valid, the user will be signed
/// in and a JWT access token and refresh token will be returned. If the email
/// and password are invalid, an error will be returned.
#[openapi(ignore = "conn", tag = "User")]
#[post("/sign-in", data = "<credentials>")]
pub async fn sign_in(credentials: Validated<Json<CredentialsDTO>>, conn: Connection<'_, Db>) {
    crate::application::commands::sign_in::action(credentials.into_deep_inner(), conn.into_inner()).await;
}
