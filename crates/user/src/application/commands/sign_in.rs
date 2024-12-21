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
    let user = UserEntity::Entity::find()
        .columns(vec![
            UserEntity::Column::Id,
            UserEntity::Column::Ban,
            UserEntity::Column::BanReason,
        ])
        .filter(UserEntity::Column::Username.eq(credentials.username))
        .find_also_related(PasswordEntity::Entity)
        .one(conn)
        .await?;

    if user.is_none() {
        return Err(Error::from(SignInError::UserNotFound));
    }

    let (user, password) = user.unwrap();
    if user.ban {
        let reason = user.ban_reason.unwrap_or("Reason not specified".to_string());
        return Err(Error::from(SignInError::UserBanned(reason)));
    }

    if password.is_none() {
        return Err(Error::from(SignInError::InvalidPassword));
    }

    let argon2 = Argon2::default();
    let password = password.unwrap();
    let password_hash = PasswordHash::new(&password.hash).map_err(|_| SignInError::InvalidPassword)?;
    let password_check = argon2.verify_password(credentials.password.as_bytes(), &password_hash);

    if password_check.is_err() {
        return Err(Error::from(SignInError::InvalidPassword));
    }

    let scopes: HashSet<String> = HashSet::from_iter(vec![]);
    let exp_ttl = 43_200; // 12 hours
    let token = jwt_auth.generate_token(user.id.to_string(), scopes, exp_ttl)?;

    Ok(Json(SignInDTO {
        access_token: token,
        refresh_token: None,
        expires_in: exp_ttl,
        token_type: "Bearer".to_string(),
        username: user.username,
        user_id: user.id,
    }))
}
