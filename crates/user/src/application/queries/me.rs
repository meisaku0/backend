use rocket::serde::json::Json;
use sea_orm::prelude::Uuid;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use shared::responses::error::Error;

use crate::domain::entities::{AvatarEntity, EmailEntity, UserEntity};
use crate::infrastructure::http::guards::auth::JwtGuard;
use crate::presentation::dto::me::UserMeDTO;

pub async fn action(conn: &DatabaseConnection, jwt_guard: JwtGuard) -> Result<Json<UserMeDTO>, Error> {
    let user = UserEntity::Entity::find()
        .filter(UserEntity::Column::Id.eq(Uuid::parse_str(&jwt_guard.claims.sub).unwrap()))
        .filter(UserEntity::Column::Ban.eq(false))
        .one(conn)
        .await?
        .unwrap();

    let email = user.find_related(EmailEntity::Entity).into_model().one(conn).await?;
    let avatar = user.find_related(AvatarEntity::Entity).into_model().all(conn).await?;

    Ok(Json(UserMeDTO {
        user: user.into(),
        email,
        avatar,
    }))
}
