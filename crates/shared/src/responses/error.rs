use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::serde_json;
use rocket::serde::Serialize;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::okapi::schemars::{self, Map};
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::OpenApiError;

/// Error messages returned to user
#[derive(Debug, Serialize, schemars::JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct Error {
    /// The title of the error message
    pub error: String,
    /// The description of the error
    pub message: Option<String>,
    /// HTTP Status Code returned
    #[serde(skip)]
    pub status_code: u16,
    /// Data returned for the error
    pub data: Option<serde_json::Value>,
}

impl OpenApiResponderInner for Error {
    fn responses(_generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        fn build_response(
            description: &str,
        ) -> rocket_okapi::okapi::openapi3::RefOr<rocket_okapi::okapi::openapi3::Response> {
            rocket_okapi::okapi::openapi3::RefOr::Object(rocket_okapi::okapi::openapi3::Response {
                description: description.to_string(),
                ..Default::default()
            })
        }

        let mut responses = Map::new();
        responses.insert(
            "400".to_string(),
            build_response(
                "# [400 Bad Request](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)\nThe request given \
                 is wrongly formatted or data asked could not be fulfilled.",
            ),
        );
        responses.insert(
            "404".to_string(),
            build_response(
                "# [404 Not Found](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)\nThis response is \
                 given when you request a page that does not exist.",
            ),
        );
        responses.insert(
            "422".to_string(),
            build_response(
                "# [422 Unprocessable Entity](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/422)\nThis \
                 response is given when your request body is not correctly formatted.",
            ),
        );
        responses.insert(
            "500".to_string(),
            build_response(
                "# [500 Internal Server Error](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)\nThis \
                 response is given when something went wrong on the server.",
            ),
        );

        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Error `{}`: {}", self.error, self.message.as_deref().unwrap_or("<no message>"))
    }
}

impl std::error::Error for Error {}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        match serde_json::to_string(&self) {
            Ok(body) => {
                Response::build()
                    .sized_body(body.len(), std::io::Cursor::new(body))
                    .header(ContentType::JSON)
                    .status(Status::new(self.status_code))
                    .ok()
            },
            Err(_) => {
                Response::build()
                    .status(Status::InternalServerError)
                    .header(ContentType::Plain)
                    .sized_body("Internal Server Error".len(), std::io::Cursor::new("Internal Server Error"))
                    .ok()
            },
        }
    }
}

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    InternalError(String),
    UnprocessableEntity(String),
}

impl From<AppError> for Error {
    fn from(app_err: AppError) -> Self {
        match app_err {
            AppError::BadRequest(msg) => {
                Error {
                    error: "Bad Request".to_string(),
                    message: Some(msg),
                    status_code: 400,
                    data: None,
                }
            },
            AppError::NotFound(msg) => {
                Error {
                    error: "Not Found".to_string(),
                    message: Some(msg),
                    status_code: 404,
                    data: None,
                }
            },
            AppError::InternalError(msg) => {
                Error {
                    error: "Internal Server Error".to_string(),
                    message: Some(msg),
                    status_code: 500,
                    data: None,
                }
            },
            AppError::UnprocessableEntity(msg) => {
                Error {
                    error: "Unprocessable Entity".to_string(),
                    message: Some(msg),
                    status_code: 422,
                    data: None,
                }
            },
        }
    }
}

impl From<rocket::serde::json::Error<'_>> for AppError {
    fn from(err: rocket::serde::json::Error) -> Self {
        use rocket::serde::json::Error::*;
        match err {
            Io(io_error) => AppError::UnprocessableEntity(io_error.to_string()),
            Parse(_, parse_error) => AppError::UnprocessableEntity(parse_error.to_string()),
        }
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(err: sea_orm::DbErr) -> Self {
        let app_error: AppError = err.into();
        app_error.into()
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        match err {
            sea_orm::DbErr::RecordNotFound(msg) => AppError::NotFound(msg),
            sea_orm::DbErr::Query(err) => AppError::InternalError(format!("Database query error: {}", err)),
            sea_orm::DbErr::Exec(err) => AppError::InternalError(format!("Execution error: {}", err)),
            _ => AppError::InternalError("An unexpected database error occurred".to_string()),
        }
    }
}
