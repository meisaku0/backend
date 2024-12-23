use std::collections::HashSet;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use auth::jwt::JwtAuth;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, TransactionTrait};
use shared::responses::error::{AppError, Error};

use crate::domain::entities::UserSessionEntity::TokenType;
use crate::domain::entities::{PasswordEntity, UserEntity, UserSessionEntity};
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
    credentials: CredentialsDTO, conn: &DatabaseConnection, jwt_auth: &State<JwtAuth>, user_ip: String,
    user_agent: String,
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
    let (token, token_refresh, session_id) =
        generate_jwt_token(jwt_auth, user.id, exp_ttl, user_ip, user_agent, conn).await?;

    Ok(Json(SignInDTO {
        access_token: token,
        refresh_token: Some(token_refresh),
        expires_in: exp_ttl,
        token_type: "Bearer".to_string(),
        username: user.username,
        user_id: user.id,
        session_id,
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

async fn generate_jwt_token(
    jwt_auth: &State<JwtAuth>, user_id: Uuid, exp_ttl: u64, ip: String, user_agent: String, conn: &DatabaseConnection,
) -> Result<(String, String, Uuid), AppError> {
    let mut scopes: HashSet<String> = HashSet::new();
    scopes.insert("access".to_string());
    let token = jwt_auth.generate_token(user_id.to_string(), scopes, exp_ttl)?;

    let user_agent_parser = uap_rust::parser::Parser::new();
    let user_agent_info = user_agent_parser.unwrap().parse(user_agent);

    let txn = conn.begin().await?;

    let session = UserSessionEntity::ActiveModel {
        user_id: ActiveValue::Set(user_id),
        ip: ActiveValue::Set(ip),
        os: ActiveValue::Set(user_agent_info.os.family),
        device: ActiveValue::Set(user_agent_info.device.brand.unwrap_or_default()),
        token_type: ActiveValue::Set(TokenType::Access),
        token: ActiveValue::Set(token.clone()),
        browser: ActiveValue::Set(user_agent_info.user_agent.family),
        ..Default::default()
    };

    let session = UserSessionEntity::Entity::insert(session).exec(&txn).await?;

    txn.commit().await?;

    let mut scopes: HashSet<String> = HashSet::new();
    scopes.insert("refresh".to_string());

    let token_refresh = jwt_auth.generate_token(session.last_insert_id.to_string(), scopes, exp_ttl * 2)?;

    Ok((token, token_refresh, session.last_insert_id))
}
