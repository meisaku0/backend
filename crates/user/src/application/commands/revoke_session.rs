use rocket::http::Status;
use sea_orm::prelude::{Expr, Uuid};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait};
use shared::responses::error::{AppError, Error};

use crate::domain::entities::UserSessionEntity;
use crate::infrastructure::http::guards::auth::JwtGuard;

pub enum RevokeSessionError {
    SessionNotFound,
    SessionNotBelongToUser,
    SessionAlreadyRevoked,
    SessionRevocationFailed,
}

impl From<RevokeSessionError> for Error {
    fn from(value: RevokeSessionError) -> Self {
        match value {
            RevokeSessionError::SessionNotFound => AppError::NotFound("Session not found".to_string()).into(),
            RevokeSessionError::SessionNotBelongToUser => {
                AppError::Unauthorized("Session not belong to user".to_string()).into()
            },
            RevokeSessionError::SessionAlreadyRevoked => {
                AppError::BadRequest("Session already revoked".to_string()).into()
            },
            RevokeSessionError::SessionRevocationFailed => {
                AppError::InternalError("Session revocation failed".to_string()).into()
            },
        }
    }
}

pub async fn action(conn: &DatabaseConnection, jwt_guard: JwtGuard, session_id: Option<&str>) -> Result<Status, Error> {
    let user_id = jwt_guard.claims.sub;

    if let Some(session) = session_id {
        let session_id =
            Uuid::parse_str(&session).map_err(|_| AppError::BadRequest("Invalid session ID".to_string()))?;

        let session = UserSessionEntity::Entity::find_by_id(session_id)
            .filter(UserSessionEntity::Column::UserId.eq(Uuid::parse_str(&user_id).unwrap()))
            .one(conn)
            .await
            .map_err(|_| RevokeSessionError::SessionNotFound)?;

        return if let Some(session) = session {
            let txn = conn.begin().await?;

            if !session.active {
                return Err(RevokeSessionError::SessionAlreadyRevoked.into());
            }

            let mut session: UserSessionEntity::ActiveModel = session.into();
            session.active = ActiveValue::Set(false);

            UserSessionEntity::Entity::update(session)
                .exec(&txn)
                .await
                .map_err(|_| RevokeSessionError::SessionRevocationFailed)?;

            txn.commit().await?;

            Ok(Status::Ok)
        } else {
            Err(RevokeSessionError::SessionNotBelongToUser.into())
        };
    }

    let txn = conn.begin().await?;

    UserSessionEntity::Entity::update_many()
        .col_expr(UserSessionEntity::Column::Active, Expr::value(false))
        .filter(UserSessionEntity::Column::UserId.eq(Uuid::parse_str(&user_id).unwrap()))
        .exec(&txn)
        .await?;

    txn.commit().await?;

    Ok(Status::Ok)
}
