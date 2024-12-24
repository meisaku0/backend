use rocket::http::Status;
use sea_orm::prelude::{DateTimeWithTimeZone, Uuid};
use sea_orm::sqlx::types::chrono;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter,
    TransactionTrait,
};
use shared::responses::error::{AppError, Error};

use crate::domain::entities::EmailEntity;
use crate::presentation::dto::active_email::ActiveEmailDTO;

#[derive(Debug)]
enum EmailActivationError {
    InvalidToken,
    DatabaseError(sea_orm::DbErr),
}

impl From<EmailActivationError> for Error {
    fn from(error: EmailActivationError) -> Self {
        match error {
            EmailActivationError::InvalidToken => AppError::BadRequest("Invalid activation token.".to_string()).into(),
            EmailActivationError::DatabaseError(err) => AppError::InternalError(err.to_string()).into(),
        }
    }
}

impl From<sea_orm::DbErr> for EmailActivationError {
    fn from(err: sea_orm::DbErr) -> Self { EmailActivationError::DatabaseError(err) }
}

pub async fn action(activation: ActiveEmailDTO, conn: &DatabaseConnection) -> Result<Status, Error> {
    let txn = conn.begin().await?;

    let mut token: EmailEntity::ActiveModel = find_valid_token(activation.token, &txn).await?.into();

    token.active = ActiveValue::Set(true);
    token.activation_token = ActiveValue::Set(Uuid::new_v4());
    token.updated_at = ActiveValue::Set(DateTimeWithTimeZone::from(chrono::Utc::now()));

    token.save(&txn).await?;

    txn.commit().await?;

    Ok(Status::Ok)
}

async fn find_valid_token(token: Uuid, conn: &DatabaseTransaction) -> Result<EmailEntity::Model, EmailActivationError> {
    let email_activation = EmailEntity::Entity::find()
        .filter(EmailEntity::Column::ActivationToken.eq(token))
        .filter(EmailEntity::Column::Active.eq(false))
        .one(conn)
        .await?
        .ok_or(EmailActivationError::InvalidToken)?;

    Ok(email_activation)
}
