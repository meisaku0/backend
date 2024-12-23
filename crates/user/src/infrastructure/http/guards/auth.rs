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

use crate::domain::entities::{UserEntity, UserSessionEntity};

#[derive(Debug)]
pub struct JwtGuard {
    pub claims: Claims,
    pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtGuard {
    type Error = AppError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = match req.headers().get_one("Authorization") {
            Some(header) => header,
            None => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AppError::Unauthorized("Missing Authorization header".into()),
                ))
            },
        };

        let token = match auth_header.strip_prefix("Bearer ") {
            Some(token) => token,
            None => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AppError::Unauthorized("Invalid Authorization header format".into()),
                ))
            },
        };

        let jwt_auth = match req.rocket().state::<JwtAuth>() {
            Some(auth) => auth,
            None => panic!("JwtAuth must be managed by Rocket"),
        };

        let data = match jwt_auth.validate_token(token) {
            Ok(data) => data,
            Err(e) => return Outcome::Error((Status::Unauthorized, e)),
        };

        let db = match req.guard::<Connection<'_, Db>>().await {
            Outcome::Success(conn) => conn.into_inner(),
            _ => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AppError::TokenValidationError("Failed to acquire database connection".into()),
                ))
            },
        };

        let user = match UserEntity::Entity::find()
            .filter(UserEntity::Column::Id.eq(Uuid::parse_str(&data.claims.sub).unwrap()))
            .one(db)
            .await
        {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AppError::TokenValidationError("Cannot validate the JWT user.".into()),
                ))
            },
            Err(_) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AppError::TokenValidationError("Database query error.".into()),
                ))
            },
        };

        if user.ban {
            let ban_reason = user.ban_reason.unwrap_or_else(|| "Reason not specified".to_string());

            return Outcome::Error((
                Status::Unauthorized,
                AppError::TokenValidationError(format!("User with id {} is banned: {}", data.claims.sub, ban_reason)),
            ));
        }

        match UserSessionEntity::Entity::find()
            .filter(UserSessionEntity::Column::UserId.eq(user.id))
            .filter(UserSessionEntity::Column::Token.eq(token))
            .one(db)
            .await
        {
            Ok(Some(session)) => {
                if !session.active {
                    return Outcome::Error((
                        Status::Unauthorized,
                        AppError::TokenValidationError("Session is not active.".into()),
                    ));
                }
            },
            Ok(None) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AppError::TokenValidationError("Cannot validate the JWT session.".into()),
                ))
            },
            Err(_) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AppError::TokenValidationError("Database query error.".into()),
                ))
            },
        };

        Outcome::Success(JwtGuard {
            claims: data.claims,
            token: token.to_string(),
        })
    }
}

#[rocket::async_trait]
impl OpenApiFromRequest<'_> for JwtGuard {
    fn from_request_input(
        _gen: &mut OpenApiGenerator, _name: String, _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some("JWT token obtained from the `/user/sign-in` endpoint.".to_owned()),
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
