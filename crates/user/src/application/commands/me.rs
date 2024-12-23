use rocket::serde::json::Json;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sea_orm::prelude::Uuid;
use shared::responses::error::{AppError, Error};

use crate::domain::entities::UserEntity;
use crate::domain::entities::UserEntity::PartialUser;
use crate::infrastructure::http::guards::auth::JwtGuard;

pub async fn action(conn: &DatabaseConnection, jwt_guard: JwtGuard) -> Result<Json<PartialUser>, Error> {
    let user_data = UserEntity::Entity::find()
        .filter(UserEntity::Column::Id.eq(Uuid::parse_str(&jwt_guard.0.sub).unwrap()))
        .filter(UserEntity::Column::Ban.eq(false))
        .into_partial_model::<PartialUser>()
        .one(conn)
        .await?
        .ok_or(AppError::InternalError(
            "There was an error obtaining the current user or the current user is not available.".to_string(),
        ))?;

    Ok(Json(user_data))
}
