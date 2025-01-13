use std::collections::HashSet;

use auth::jwt::JwtAuth;
use email::ResendMailer;
use rocket::futures::TryFutureExt;
use rocket::http::uri::Absolute;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::State;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use shared::responses::error::{AppError, Error};

use crate::domain::entities::{EmailEntity, PasswordEntity, UserEntity};

enum ResetPasswordErrors {
    UserNotFound,
    PasswordNotFound,
    InternalError,
    SendEmailError(String),
}

impl From<ResetPasswordErrors> for Error {
    fn from(error: ResetPasswordErrors) -> Self {
        match error {
            ResetPasswordErrors::UserNotFound => AppError::NotFound("User not found".to_string()).into(),
            ResetPasswordErrors::PasswordNotFound => AppError::InternalError("Password not found".to_string()).into(),
            ResetPasswordErrors::InternalError => AppError::InternalError("Internal error".to_string()).into(),
            ResetPasswordErrors::SendEmailError(e) => AppError::InternalError(e).into(),
        }
    }
}

pub async fn action(
    username: &str, conn: &DatabaseConnection, resend_mailer: ResendMailer, jwt_auth: &State<JwtAuth>,
    base_api_url: String,
) -> Result<Status, Error> {
    let user = UserEntity::Entity::find()
        .filter(UserEntity::Column::Username.eq(username))
        .find_also_related(EmailEntity::Entity)
        .one(conn)
        .await?;

    if user.is_none() {
        return Err(ResetPasswordErrors::UserNotFound.into());
    }

    let (user, email) = user.unwrap();

    let user_password = PasswordEntity::Entity::find()
        .filter(PasswordEntity::Column::UserId.eq(user.id))
        .one(conn)
        .await?;

    if user_password.is_none() {
        return Err(ResetPasswordErrors::PasswordNotFound.into());
    }

    let reset_token = Uuid::new_v4();
    let mut user_password: PasswordEntity::ActiveModel = user_password.unwrap().into();
    user_password.password_reset_token = ActiveValue::Set(Some(reset_token));

    let mut scopes: HashSet<String> = HashSet::new();
    scopes.insert("reset-password".to_string());
    let jwt_token = jwt_auth.generate_token(reset_token.to_string(), scopes, 3600)?;

    let reset_password_url = format!("{}/user/reset-password?token={}", base_api_url, jwt_token);
    let reset_password_url = Absolute::parse(&reset_password_url).map_err(|_| ResetPasswordErrors::InternalError)?;

    resend_mailer
        .send_email(
            vec![&email.unwrap().key],
            "Reset password for your Meisaku account",
            "reset_password",
            json!({ "user_name": user.username, "reset_link": reset_password_url.to_string() }),
        )
        .map_err(|e| ResetPasswordErrors::SendEmailError(e.to_string()))
        .await?;

    Ok(Status::Ok)
}
