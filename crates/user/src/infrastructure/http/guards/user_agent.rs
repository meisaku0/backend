use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use shared::responses::error::AppError;

pub struct UserAgent(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent {
    type Error = AppError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("User-Agent") {
            Some(user_agent) => Outcome::Success(UserAgent(user_agent.to_string())),
            None => Outcome::Error((Status::BadRequest, AppError::BadRequest("User-Agent header is required".to_string()))),
        }
    }
}
