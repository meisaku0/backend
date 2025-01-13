use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use rocket::http::Status;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait};
use shared::responses::error::{AppError, Error};

use crate::application::commands::sign_in::SignInError;
use crate::domain::entities::PasswordEntity;
use crate::infrastructure::http::guards::auth::JwtGuard;
use crate::presentation::dto::change_password::ChangePasswordDTO;

pub enum ChangePasswordError {
    PasswordNotMatch,
    PasswordNotAvailable,
}

impl From<ChangePasswordError> for Error {
    fn from(value: ChangePasswordError) -> Self {
        match value {
            ChangePasswordError::PasswordNotMatch => AppError::Unauthorized("Password not match.".to_string()).into(),
            ChangePasswordError::PasswordNotAvailable => {
                AppError::Unauthorized("Password not available or disabled.".to_string()).into()
            },
        }
    }
}

pub async fn action(
    change_password_dto: ChangePasswordDTO, conn: &DatabaseConnection, jwt_guard: JwtGuard,
) -> Result<Status, Error> {
    let user_password = PasswordEntity::Entity::find()
        .filter(PasswordEntity::Column::UserId.eq(Uuid::parse_str(&jwt_guard.claims.sub).unwrap()))
        .one(conn)
        .await
        .map_err(|_| ChangePasswordError::PasswordNotAvailable)?;

    if user_password.is_none() {
        return Err(ChangePasswordError::PasswordNotAvailable.into());
    }

    let argon2 = argon2::Argon2::default();
    let user_password = user_password.unwrap();

    let password_hash = PasswordHash::new(&user_password.hash).map_err(|_| SignInError::InvalidPassword)?;

    argon2
        .verify_password(change_password_dto.current_password.as_bytes(), &password_hash)
        .map_err(|_| ChangePasswordError::PasswordNotMatch)?;

    let salt = SaltString::generate(&mut OsRng);
    let new_password_hash = argon2
        .hash_password(change_password_dto.new_password.as_bytes(), &salt)
        .unwrap();

    let txn = conn.begin().await?;

    let mut user_password: PasswordEntity::ActiveModel = user_password.into();
    user_password.hash = ActiveValue::Set(new_password_hash.to_string());
    user_password.salt = ActiveValue::Set(salt.to_string());
    user_password.password_reset_token = ActiveValue::Set(None);

    PasswordEntity::Entity::update(user_password).exec(&txn).await?;

    txn.commit().await?;

    Ok(Status::Ok)
}
