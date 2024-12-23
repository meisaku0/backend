use std::net::IpAddr;

use auth::jwt::JwtAuth;
use config::database::pool::Db;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, patch, post, State};
use rocket_okapi::openapi;
use rocket_validation::Validated;
use sea_orm_rocket::Connection;
use shared::responses::error::Error;
use crate::domain::entities::UserEntity::PartialUser;
use crate::infrastructure::http::guards::auth::JwtGuard;
use crate::infrastructure::http::guards::user_agent::UserAgent;
use crate::presentation::dto::active_email::ActiveEmailDTO;
use crate::presentation::dto::create_user::{CreateUserDTO, UserCreatedDTO};
use crate::presentation::dto::refresh_session::RefreshSessionDTO;
use crate::presentation::dto::sign_in::{CredentialsDTO, SignInDTO};

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
#[openapi(ignore = "conn", tag = "User", ignore = "user_agent")]
#[post("/sign-in", data = "<credentials>")]
pub async fn sign_in(
    credentials: Validated<Json<CredentialsDTO>>, conn: Connection<'_, Db>, jwt_auth: &State<JwtAuth>,
    client_ip: IpAddr, user_agent: UserAgent,
) -> Result<Json<SignInDTO>, Error> {
    crate::application::commands::sign_in::action(
        credentials.into_deep_inner(),
        conn.into_inner(),
        jwt_auth,
        client_ip.to_string(),
        user_agent.0,
    )
    .await
}

/// # Refresh session
///
/// This endpoint is used to refresh a user's session. The user must provide
/// their refresh token. If the refresh token is valid, a new JWT access token
/// and refresh token will be returned. If the refresh token is invalid, an
/// error will be returned.
#[openapi(ignore = "conn", tag = "User", ignore = "user_agent")]
#[post("/refresh-session", data = "<refresh>")]
pub async fn refresh_session(
    refresh: Validated<Json<RefreshSessionDTO>>, conn: Connection<'_, Db>, jwt_auth: &State<JwtAuth>,
    client_ip: IpAddr, user_agent: UserAgent,
) -> Result<Json<SignInDTO>, Error> {
    crate::application::commands::refresh_session::action(
        refresh.into_deep_inner(),
        conn.into_inner(),
        jwt_auth,
        client_ip.to_string(),
        user_agent.0,
    )
    .await
}

/// # Me
///
/// This endpoint is used to get the user's information. The user must provide
/// their JWT access token. If the JWT access token is valid, the user's
/// information will be returned. If the JWT access token is invalid, an error
/// will be returned.
#[openapi(ignore = "conn", tag = "User")]
#[get("/me")]
pub async fn me(conn: Connection<'_, Db>, jwt_guard: JwtGuard) -> Result<Json<PartialUser>, Error> {
    crate::application::commands::me::action(conn.into_inner(), jwt_guard).await
}
