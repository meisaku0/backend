use rocket::serde::json::Json;
use sea_orm::prelude::Uuid;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use shared::responses::error::Error;

use crate::domain::entities::{EmailEntity, UserEntity};
use crate::infrastructure::http::guards::auth::JwtGuard;
use crate::presentation::dto::me::UserMeDTO;

pub async fn action(conn: &DatabaseConnection, jwt_guard: JwtGuard) -> Result<Json<UserMeDTO>, Error> {
    let (user, email) = UserEntity::Entity::find()
        .filter(UserEntity::Column::Id.eq(Uuid::parse_str(&jwt_guard.claims.sub).unwrap()))
        .filter(UserEntity::Column::Ban.eq(false))
        .find_also_related(EmailEntity::Entity)
        .into_model()
        .one(conn)
        .await?
        .unwrap();

    Ok(Json(UserMeDTO {
        user: Some(user),
        email,
    }))
}
