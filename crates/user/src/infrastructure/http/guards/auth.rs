use auth::jwt::{Claims, JwtAuth};
use config::database::pool::Db;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use sea_orm::prelude::Uuid;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm_rocket::Connection;
use shared::responses::error::AppError;

use crate::domain::entities::UserEntity;

#[derive(Debug)]
pub struct JwtGuard(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtGuard {
    type Error = AppError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = req.headers().get_one("Authorization");

        if let Some(auth_value) = auth_header {
            if let Some(token) = auth_value.strip_prefix("Bearer ") {
                let jwt_auth = req
                    .rocket()
                    .state::<JwtAuth>()
                    .expect("JwtAuth must be managed by Rocket");

                match jwt_auth.validate_token(token) {
                    Ok(data) => {
                        let db = req
                            .guard::<Connection<'_, Db>>()
                            .await
                            .expect("Database connection must be managed by Rocket");
                        let db = db.into_inner();

                        match UserEntity::Entity::find()
                            .filter(UserEntity::Column::Id.eq(Uuid::parse_str(&data.claims.sub).unwrap()))
                            .one(db)
                            .await
                        {
                            Ok(user) => {
                                if let Some(user) = user {
                                    if user.ban {
                                        let ban_reason = user.ban_reason.unwrap_or("Reason not specified".to_string());

                                        return Outcome::Error((
                                            Status::Unauthorized,
                                            AppError::TokenValidationError(format!(
                                                "User with id {} is banned: {}",
                                                data.claims.sub, ban_reason
                                            )),
                                        ));
                                    }
                                } else {
                                    return Outcome::Error((
                                        Status::Unauthorized,
                                        AppError::TokenValidationError("Cannot validate the JWT user.".into()),
                                    ));
                                }

                                Outcome::Success(JwtGuard(data.claims))
                            },
                            Err(_) => {
                                Outcome::Error((
                                    Status::Unauthorized,
                                    AppError::TokenValidationError("Cannot validate the JWT user.".into()),
                                ))
                            },
                        }
                    },
                    Err(e) => Outcome::Error((Status::Unauthorized, e)),
                }
            } else {
                Outcome::Error((
                    Status::Unauthorized,
                    AppError::Unauthorized("Invalid Authorization header format".into()),
                ))
            }
        } else {
            Outcome::Error((Status::Unauthorized, AppError::Unauthorized("Missing Authorization header".into())))
        }
    }
}

#[rocket::async_trait]
impl OpenApiFromRequest<'_> for JwtGuard {
    fn from_request_input(
        _gen: &mut OpenApiGenerator, _name: String, _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some(
                "JWT token obtained from the /auth endpoint. The token must be prefixed with 'Bearer '".to_owned(),
            ),
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(),
                bearer_format: Some("JWT".to_owned()),
            },
            extensions: Object::default(),
        };

        let mut security_req = SecurityRequirement::new();
        security_req.insert("JwtAuth".to_owned(), Vec::new());

        Ok(RequestHeaderInput::Security("JwtAuth".to_owned(), security_scheme, security_req))
    }
}
