use auth::jwt::{Claims, JwtAuth};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
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
                            .rocket()
                            .state::<DatabaseConnection>()
                            .expect("Database must be managed by Rocket");

                        let user_exist = UserEntity::Entity::find()
                            .filter(UserEntity::Column::Id.eq(data.claims.sub.clone()))
                            .one(db)
                            .await;

                        if user_exist.is_err() {
                            return Outcome::Error((
                                Status::Unauthorized,
                                AppError::TokenValidationError("An error occurred when validating user token.".into()),
                            ));
                        }

                        Outcome::Success(JwtGuard(data.claims))
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
impl<'a> OpenApiFromRequest<'a> for JwtGuard {
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
