use std::collections::HashSet;

use auth::jwt::JwtAuth;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait};
use shared::responses::error::{AppError, Error};

use crate::domain::entities::user_session::TokenType;
use crate::domain::entities::UserSessionEntity;
use crate::presentation::dto::refresh_session::RefreshSessionDTO;
use crate::presentation::dto::sign_in::SignInDTO;

#[derive(Debug)]
pub enum RefreshSessionError {
    InvalidAccessToken,
    InvalidRefreshToken,
}

impl From<RefreshSessionError> for Error {
    fn from(error: RefreshSessionError) -> Self {
        match error {
            RefreshSessionError::InvalidAccessToken => {
                AppError::Unauthorized("The associated access token is inactive or invalid.".to_string()).into()
            },
            RefreshSessionError::InvalidRefreshToken => {
                AppError::BadRequest("The refresh token is invalid.".to_string()).into()
            },
        }
    }
}

pub async fn action(
    refresh_session: RefreshSessionDTO, conn: &DatabaseConnection, jwt_auth: &State<JwtAuth>, user_ip: String,
    user_agent: String,
) -> Result<Json<SignInDTO>, Error> {
    let user_agent_parser = uap_rust::parser::Parser::new();
    let user_agent_info = user_agent_parser.unwrap().parse(user_agent);

    let token = jwt_auth
        .validate_token(&refresh_session.refresh_token)
        .map_err(|_| RefreshSessionError::InvalidRefreshToken)?;

    if !token.claims.scopes.contains("refresh") {
        return Err(RefreshSessionError::InvalidRefreshToken.into());
    }

    let token_session = UserSessionEntity::Entity::find()
        .filter(UserSessionEntity::Column::TokenType.eq(TokenType::Access))
        .filter(UserSessionEntity::Column::Id.eq(Uuid::parse_str(&token.claims.sub).unwrap()))
        .filter(UserSessionEntity::Column::Active.eq(true))
        .one(conn)
        .await?
        .ok_or(RefreshSessionError::InvalidAccessToken)?;

    let token_ttl = 43_200;
    let mut scopes: HashSet<String> = HashSet::new();
    scopes.insert("access".to_string());

    let access_token = jwt_auth
        .generate_token(token_session.user_id.to_string(), scopes, token_ttl)
        .map_err(|_| RefreshSessionError::InvalidAccessToken)?;

    let txn = conn.begin().await?;

    let session = UserSessionEntity::ActiveModel {
        user_id: ActiveValue::Set(token_session.user_id),
        ip: ActiveValue::Set(user_ip),
        os: ActiveValue::Set(user_agent_info.os.family),
        device: ActiveValue::Set(user_agent_info.device.brand.unwrap_or_default()),
        token_type: ActiveValue::Set(TokenType::Access),
        token: ActiveValue::Set(access_token.clone()),
        browser: ActiveValue::Set(user_agent_info.user_agent.family),
        ..Default::default()
    };

    let session = UserSessionEntity::Entity::insert(session).exec(&txn).await?;

    let mut old_token_session: UserSessionEntity::ActiveModel = token_session.into();
    old_token_session.active = ActiveValue::Set(false);

    UserSessionEntity::Entity::update(old_token_session.clone())
        .exec(&txn)
        .await?;

    txn.commit().await?;

    Ok(Json(SignInDTO {
        access_token,
        refresh_token: Some(refresh_session.refresh_token),
        expires_in: token_ttl,
        token_type: "Bearer".to_string(),
        username: None,
        user_id: old_token_session.user_id.unwrap(),
        session_id: session.last_insert_id,
    }))
}
