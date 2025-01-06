use sea_orm::prelude::Uuid;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait};
use shared::responses::error::{AppError, Error};

use crate::domain::entities::session::TokenType;
use crate::domain::entities::SessionEntity;
use crate::infrastructure::http::guards::auth::JwtGuard;

pub async fn action(conn: &DatabaseConnection, jwt_data: JwtGuard) -> Result<(), Error> {
    let session = SessionEntity::Entity::find()
        .filter(SessionEntity::Column::UserId.eq(Uuid::parse_str(&jwt_data.claims.sub).unwrap()))
        .filter(SessionEntity::Column::Token.eq(jwt_data.token))
        .filter(SessionEntity::Column::TokenType.eq(TokenType::Access))
        .one(conn)
        .await?
        .ok_or(AppError::Unauthorized("Session not found".to_string()))?;

    let tx = conn.begin().await?;

    let mut session: SessionEntity::ActiveModel = session.into();
    session.active = ActiveValue::Set(false);

    SessionEntity::Entity::update(session).exec(&tx).await?;

    tx.commit().await?;

    Ok(())
}
