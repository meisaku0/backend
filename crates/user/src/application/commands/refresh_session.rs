use auth::jwt::JwtAuth;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::DatabaseConnection;
use shared::responses::error::{AppError, Error};

use crate::presentation::dto::refresh_session::RefreshSessionDTO;
use crate::presentation::dto::sign_in::SignInDTO;

pub async fn action(
    refresh_session: RefreshSessionDTO, conn: &DatabaseConnection, jwt_auth: &State<JwtAuth>, user_ip: String,
    user_agent: String,
) -> Result<Json<SignInDTO>, Error> {
    Err(AppError::ExpiredToken.into())
}
