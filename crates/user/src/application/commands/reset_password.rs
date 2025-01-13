use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use auth::jwt::JwtAuth;
use rocket::http::Status;
use rocket::State;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait};
use shared::responses::error::{AppError, Error};

use crate::domain::entities::PasswordEntity;
use crate::presentation::dto::reset_password::ResetPasswordDTO;

enum ResetPasswordError {
    InvalidToken,
    PasswordHashingError(String),
}

impl From<ResetPasswordError> for Error {
    fn from(error: ResetPasswordError) -> Self {
        match error {
            ResetPasswordError::InvalidToken => AppError::Unauthorized("Invalid token".to_string()).into(),
            ResetPasswordError::PasswordHashingError(msg) => AppError::InternalError(msg).into(),
        }
    }
}

pub async fn action(
    reset_password: ResetPasswordDTO, conn: &DatabaseConnection, jwt_auth: &State<JwtAuth>,
) -> Result<Status, Error> {
    let txn = conn.begin().await?;
    let token = jwt_auth.validate_token(&reset_password.reset_token)?;
    let user_password = PasswordEntity::Entity::find()
        .filter(PasswordEntity::Column::ResetToken.eq(Uuid::parse_str(&token.claims.sub).unwrap()))
        .one(conn)
        .await?;

    if user_password.is_none() {
        return Err(ResetPasswordError::InvalidToken.into());
    }

    let new_password = create_password_record(reset_password.new_password).await?;
    let mut user_password: PasswordEntity::ActiveModel = user_password.unwrap().into();
    user_password.reset_token = ActiveValue::Set(None);
    user_password.hash = ActiveValue::Set(new_password.0);
    user_password.salt = ActiveValue::Set(new_password.1);

    PasswordEntity::Entity::update(user_password).exec(&txn).await?;

    txn.commit().await?;

    Ok(Status::Ok)
}

async fn create_password_record(new_password: String) -> Result<(String, String), ResetPasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(new_password.as_bytes(), &salt)
        .map_err(|e| ResetPasswordError::PasswordHashingError(e.to_string()))?
        .to_string();

    Ok((password_hash, salt.to_string()))
}
