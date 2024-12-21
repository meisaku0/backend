use std::collections::HashSet;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use auth::jwt::JwtAuth;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use shared::responses::error::{AppError, Error};

use crate::domain::entities::{PasswordEntity, UserEntity};
use crate::presentation::dto::sign_in::{CredentialsDTO, SignInDTO};

#[derive(Debug)]
pub enum SignInError {
    UserNotFound,
    InvalidPassword,
    UserBanned(String),
}

impl From<SignInError> for Error {
    fn from(error: SignInError) -> Self {
        match error {
            SignInError::InvalidPassword => AppError::Unauthorized("Invalid password".to_string()).into(),
            SignInError::UserNotFound => {
                AppError::Unauthorized("User not found. Check user credentials.".to_string()).into()
            },
            SignInError::UserBanned(reason) => AppError::BadRequest(format!("User is banned: {}", reason)).into(),
        }
    }
}

pub async fn action(
    credentials: CredentialsDTO, conn: &DatabaseConnection, jwt_auth: &State<JwtAuth>,
) -> Result<Json<SignInDTO>, Error> {
    let (user, password) = fetch_user_with_password(&credentials.username, conn).await?;
    if user.ban {
        let reason = user.ban_reason.unwrap_or("Reason not specified".to_string());
        return Err(Error::from(SignInError::UserBanned(reason)));
    }

    if password.is_none() {
        return Err(SignInError::InvalidPassword.into());
    }

    verify_password(&credentials.password, &password.unwrap().hash)?;

    let exp_ttl = 43_200;
    let token = generate_jwt_token(jwt_auth, user.id.to_string(), exp_ttl)?;

    Ok(Json(SignInDTO {
        access_token: token,
        refresh_token: None,
        expires_in: exp_ttl,
        token_type: "Bearer".to_string(),
        username: user.username,
        user_id: user.id,
    }))
}

async fn fetch_user_with_password(
    username: &str, conn: &DatabaseConnection,
) -> Result<(UserEntity::Model, Option<PasswordEntity::Model>), Error> {
    UserEntity::Entity::find()
        .columns(vec![
            UserEntity::Column::Id,
            UserEntity::Column::Ban,
            UserEntity::Column::BanReason,
        ])
        .filter(UserEntity::Column::Username.eq(username))
        .find_also_related(PasswordEntity::Entity)
        .one(conn)
        .await?
        .ok_or_else(|| SignInError::UserNotFound.into())
}

fn verify_password(password: &str, hashed_password: &str) -> Result<(), Error> {
    let password_hash = PasswordHash::new(hashed_password).map_err(|_| SignInError::InvalidPassword)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .map_err(|_| SignInError::InvalidPassword)?;
    Ok(())
}

fn generate_jwt_token(jwt_auth: &State<JwtAuth>, user_id: String, exp_ttl: u64) -> Result<String, AppError> {
    let scopes: HashSet<String> = HashSet::new();
    jwt_auth.generate_token(user_id, scopes, exp_ttl)
}
