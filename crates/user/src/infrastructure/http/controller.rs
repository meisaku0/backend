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

use crate::infrastructure::http::guards::auth::JwtGuard;
use crate::infrastructure::http::guards::user_agent::UserAgent;
use crate::presentation::dto::active_email::ActiveEmailDTO;
use crate::presentation::dto::change_password::ChangePasswordDTO;
use crate::presentation::dto::create_user::{CreateUserDTO, UserCreatedDTO};
use crate::presentation::dto::me::UserMeDTO;
use crate::presentation::dto::pagination::ItemPaginationDTO;
use crate::presentation::dto::refresh_session::RefreshSessionDTO;
use crate::presentation::dto::sessions::UserSessionPaginateDTO;
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
pub async fn me(conn: Connection<'_, Db>, jwt_guard: JwtGuard) -> Result<Json<UserMeDTO>, Error> {
    crate::application::queries::me::action(conn.into_inner(), jwt_guard).await
}

/// # Sign Out
///
/// This endpoint is used to sign out a user. The user must provide their JWT
/// access token to get the session id. If the JWT access token is valid, the
/// user will be signed out. If the JWT access token is invalid, an error will
/// be returned.
#[openapi(ignore = "conn", tag = "User")]
#[post("/sign-out")]
pub async fn sign_out(conn: Connection<'_, Db>, jwt_guard: JwtGuard) -> Result<(), Error> {
    crate::application::commands::sign_out::action(conn.into_inner(), jwt_guard).await
}

/// # Sessions
///
/// This endpoint is used to get the user's active sessions. The user must
/// provide their JWT access token. If the JWT access token is valid, the user's
/// sessions will be returned. If the JWT access token is invalid, an error
/// will be returned.
#[openapi(ignore = "conn", tag = "User")]
#[get("/sessions?<pagination..>")]
pub async fn sessions(
    conn: Connection<'_, Db>, jwt_guard: JwtGuard, pagination: Validated<UserSessionPaginateDTO>,
) -> Result<Json<ItemPaginationDTO>, Error> {
    crate::application::queries::sessions::action(conn.into_inner(), jwt_guard, pagination.into_inner()).await
}

/// # Revoke All Sessions
///
/// This endpoint is used to revoke all user sessions. The user must provide
/// their JWT access token. If the JWT access token is valid, all user sessions
/// will be revoked. If the JWT access token is invalid, an error will be
/// returned.
#[openapi(ignore = "conn", tag = "User")]
#[patch("/revoke-session")]
pub async fn revoke_session(conn: Connection<'_, Db>, jwt_guard: JwtGuard) -> Result<Status, Error> {
    crate::application::commands::revoke_session::action(conn.into_inner(), jwt_guard, None).await
}

/// # Revoke Session
///
/// This endpoint is used to revoke a user session. The user must provide their
/// JWT access token and the session id. If the JWT access token is valid and
/// the session id is valid, the user session will be revoked. If the JWT
/// access token is invalid or the session id is invalid, an error will be
/// returned.
#[openapi(ignore = "conn", tag = "User")]
#[patch("/revoke-session/<session_id>")]
pub async fn revoke_session_by_id(
    session_id: &str, conn: Connection<'_, Db>, jwt_guard: JwtGuard,
) -> Result<Status, Error> {
    crate::application::commands::revoke_session::action(conn.into_inner(), jwt_guard, Some(session_id)).await
}

/// # Change password
///
/// This endpoint is used to change the user's password. The user must provide
/// their JWT access token and the old password and new password. If the JWT
/// access token is valid and the old password is correct, the user's password
/// will be changed. If the JWT access token is invalid or the old password is
/// incorrect, an error will be returned.
#[openapi(ignore = "conn", tag = "User")]
#[post("/change-password", data = "<password>")]
pub async fn change_password(
    password: Validated<Json<ChangePasswordDTO>>, conn: Connection<'_, Db>, jwt_guard: JwtGuard,
) -> Result<Status, Error> {
    crate::application::commands::change_password::action(password.into_deep_inner(), conn.into_inner(), jwt_guard)
        .await
}
