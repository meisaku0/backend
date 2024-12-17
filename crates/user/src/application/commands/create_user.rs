use rocket::serde::json::Json;
use rocket_validation::Validated;
use sea_orm_rocket::Connection;
use config::database::pool::Db;
use shared::responses::error::Error;
use crate::presentation::dto::{CreateUserDTO, UserCreatedDTO};

pub fn action(user: Validated<Json<CreateUserDTO>>, conn: Connection<'_, Db>) -> Result<Json<UserCreatedDTO>, Error> { 
    let user = user.into_inner();
    
    Ok(Json(UserCreatedDTO {
        id: 1,
        username: user.username.clone(),
        email: user.email.clone(),
    }))
}