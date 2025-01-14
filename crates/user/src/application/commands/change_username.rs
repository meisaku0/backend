use rocket::http::Status;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait};
use shared::responses::error::{AppError, Error};

use crate::domain::entities::UserEntity;
use crate::domain::entities::UserEntity::PartialUser;
use crate::infrastructure::http::guards::auth::JwtGuard;
use crate::presentation::dto::change_username::ChangeUsernameDTO;

#[derive(Debug)]
enum ChangeUsernameErrors {
    UsernameNotAvailable(String),
}

impl From<ChangeUsernameErrors> for Error {
    fn from(value: ChangeUsernameErrors) -> Self {
        match value {
            ChangeUsernameErrors::UsernameNotAvailable(username) => {
                AppError::BadRequest(format!("Username {} is not available.", username)).into()
            },
        }
    }
}

pub async fn action(
    conn: &DatabaseConnection, jwt_guard: JwtGuard, change_username: ChangeUsernameDTO,
) -> Result<Status, Error> {
    let txn = conn.begin().await?;

    if let Some(user_exist) = UserEntity::Entity::find()
        .filter(UserEntity::Column::Id.ne(Uuid::parse_str(&jwt_guard.claims.sub).unwrap()))
        .filter(UserEntity::Column::Username.eq(change_username.username.clone()))
        .into_model::<PartialUser>()
        .one(&txn)
        .await?
    {
        return Err(Error::from(ChangeUsernameErrors::UsernameNotAvailable(user_exist.username)));
    };

    let user = UserEntity::Entity::find()
        .filter(UserEntity::Column::Id.eq(Uuid::parse_str(&jwt_guard.claims.sub).unwrap()))
        .one(&txn)
        .await?;

    if let Some(user) = user {
        let mut user: UserEntity::ActiveModel = user.into();

        user.username = ActiveValue::Set(change_username.username);
        UserEntity::Entity::update(user).exec(&txn).await?;
    } else {
        return Err(Error::from(AppError::InternalError("User not found.".to_string())));
    }

    Ok(Status::Ok)
}
