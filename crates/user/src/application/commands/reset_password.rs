use rocket::http::Status;
use shared::responses::error::Error;

pub fn action() -> Result<Status, Error> {
    Ok(Status::Ok)
}
