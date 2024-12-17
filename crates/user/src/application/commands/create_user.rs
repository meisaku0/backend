use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use config::database::pool::Db;
use rand::rngs::OsRng;
use rocket::serde::json::Json;
use rocket_validation::Validated;
use sea_orm::prelude::Uuid;
use sea_orm::{
    ActiveValue, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait, QueryFilter, TransactionTrait,
};
use sea_orm_rocket::Connection;
use shared::responses::error::{AppError, Error};

use crate::domain::entities::{EmailEntity, PasswordEntity, UserEntity};
use crate::presentation::dto::create_user::{CreateUserDTO, UserCreatedDTO};

#[derive(Debug)]
pub enum UserCreationError {
    DuplicateUsername,
    DuplicateEmail,
    PasswordHashingError(String),
    DatabaseError(sea_orm::DbErr),
}

impl From<UserCreationError> for Error {
    fn from(error: UserCreationError) -> Self {
        match error {
            UserCreationError::DuplicateUsername => AppError::BadRequest("Username already exists".to_string()).into(),
            UserCreationError::DuplicateEmail => AppError::BadRequest("Email already exists".to_string()).into(),
            UserCreationError::PasswordHashingError(msg) => AppError::InternalError(msg).into(),
            UserCreationError::DatabaseError(err) => AppError::InternalError(err.to_string()).into(),
        }
    }
}

impl From<sea_orm::DbErr> for UserCreationError {
    fn from(err: sea_orm::DbErr) -> Self { UserCreationError::DatabaseError(err) }
}

pub async fn action(
    user: Validated<Json<CreateUserDTO>>, conn: Connection<'_, Db>,
) -> Result<Json<UserCreatedDTO>, Error> {
    let user = user.into_inner();
    let conn = &conn.into_inner().clone();

    check_existing_user(&user, conn).await?;

    let txn = conn.begin().await?;

    match create_user_with_credentials(&user, &txn).await {
        Ok(user_data) => {
            txn.commit().await?;
            Ok(Json(user_data))
        },
        Err(e) => {
            let _ = txn.rollback().await;
            Err(e.into())
        },
    }
}

async fn check_existing_user(user: &CreateUserDTO, conn: &impl ConnectionTrait) -> Result<(), UserCreationError> {
    let user_exist = UserEntity::Entity::find()
        .filter(UserEntity::Column::Username.eq(&user.username))
        .one(conn)
        .await?;

    if user_exist.is_some() {
        return Err(UserCreationError::DuplicateUsername);
    }

    let email_exist = EmailEntity::Entity::find()
        .filter(EmailEntity::Column::Key.eq(&user.email))
        .one(conn)
        .await?;

    if email_exist.is_some() {
        return Err(UserCreationError::DuplicateEmail);
    }

    Ok(())
}

async fn create_user_with_credentials(
    user: &CreateUserDTO, txn: &DatabaseTransaction,
) -> Result<UserCreatedDTO, UserCreationError> {
    let user_db = create_user_record(user, txn).await?;
    let email_db = create_email_record(user, user_db.last_insert_id, txn).await?;
    let password_db = create_password_record(user, user_db.last_insert_id, txn).await?;

    let updated_user =
        update_user_credentials(user_db.last_insert_id, email_db.last_insert_id, password_db.last_insert_id, txn)
            .await?;

    Ok(UserCreatedDTO {
        id: updated_user.id,
        username: updated_user.username,
        email_id: updated_user.email_id.unwrap(),
        password_id: updated_user.password_id.unwrap(),
    })
}

async fn create_user_record(
    user: &CreateUserDTO, txn: &DatabaseTransaction,
) -> Result<sea_orm::InsertResult<UserEntity::ActiveModel>, UserCreationError> {
    Ok(UserEntity::Entity::insert(UserEntity::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        username: ActiveValue::Set(user.username.clone()),
        ..Default::default()
    })
    .exec(txn)
    .await?)
}

async fn create_email_record(
    user: &CreateUserDTO, user_id: Uuid, txn: &DatabaseTransaction,
) -> Result<sea_orm::InsertResult<EmailEntity::ActiveModel>, UserCreationError> {
    Ok(EmailEntity::Entity::insert(EmailEntity::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        key: ActiveValue::Set(user.email.clone()),
        active: ActiveValue::Set(false),
        activation_token: ActiveValue::Set(Uuid::new_v4()),
        user_id: ActiveValue::Set(user_id),
        ..Default::default()
    })
    .exec(txn)
    .await?)
}

async fn create_password_record(
    user: &CreateUserDTO, user_id: Uuid, txn: &DatabaseTransaction,
) -> Result<sea_orm::InsertResult<PasswordEntity::ActiveModel>, UserCreationError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(user.password.as_bytes(), &salt)
        .map_err(|e| UserCreationError::PasswordHashingError(e.to_string()))?
        .to_string();

    Ok(PasswordEntity::Entity::insert(PasswordEntity::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        active: ActiveValue::Set(true),
        activation_token: ActiveValue::Set(Uuid::new_v4()),
        user_id: ActiveValue::Set(user_id),
        hash: ActiveValue::Set(password_hash),
        salt: ActiveValue::Set(salt.to_string()),
        ..Default::default()
    })
    .exec(txn)
    .await?)
}

async fn update_user_credentials(
    user_id: Uuid, email_id: Uuid, password_id: Uuid, txn: &DatabaseTransaction,
) -> Result<UserEntity::Model, UserCreationError> {
    let user = UserEntity::Entity::find_by_id(user_id)
        .one(txn)
        .await?
        .ok_or_else(|| UserCreationError::DatabaseError(sea_orm::DbErr::Custom("User not found".to_string())))?;

    let mut user: UserEntity::ActiveModel = user.into();
    user.email_id = ActiveValue::Set(Some(email_id));
    user.password_id = ActiveValue::Set(Some(password_id));

    Ok(UserEntity::Entity::update(user).exec(txn).await?)
}
