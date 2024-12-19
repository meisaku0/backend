use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::structs::{AuthError, Claims, JwtAuth};

#[derive(Debug)]
pub struct JwtGuard(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtGuard {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = req.headers().get_one("Authorization");

        if let Some(auth_value) = auth_header {
            if let Some(token) = auth_value.strip_prefix("Bearer ") {
                let jwt_auth = req
                    .rocket()
                    .state::<JwtAuth>()
                    .expect("JwtAuth must be managed by Rocket");
                match jwt_auth.validate_token(token) {
                    Ok(data) => Outcome::Success(JwtGuard(data.claims)),
                    Err(e) => Outcome::Error((Status::Unauthorized, e)),
                }
            } else {
                Outcome::Error((
                    Status::Unauthorized,
                    AuthError::InternalError("Invalid Authorization header format".into()),
                ))
            }
        } else {
            Outcome::Error((Status::Unauthorized, AuthError::InternalError("Missing Authorization header".into())))
        }
    }
}
