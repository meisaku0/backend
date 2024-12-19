use std::collections::HashSet;
use std::sync::Arc;

use jsonwebtoken::{Algorithm, Validation};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{response, Request, Response};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub scopes: HashSet<String>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum AuthError {
    TokenCreationError(String),
    TokenValidationError(String),
    MissingScope(String),
    ExpiredToken,
    InvalidTime,
    InternalError(String),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::TokenCreationError(e) => write!(f, "Token creation error: {}", e),
            AuthError::TokenValidationError(e) => write!(f, "Token validation error: {}", e),
            AuthError::MissingScope(scope) => write!(f, "Missing required scope: {}", scope),
            AuthError::ExpiredToken => write!(f, "Token has expired"),
            AuthError::InvalidTime => write!(f, "Invalid system time"),
            AuthError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl<'r> Responder<'r, 'static> for AuthError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status_code = match &self {
            AuthError::TokenCreationError(_) => Status::InternalServerError,
            AuthError::TokenValidationError(_) => Status::Unauthorized,
            AuthError::MissingScope(_) => Status::Forbidden,
            AuthError::ExpiredToken => Status::Unauthorized,
            AuthError::InvalidTime => Status::InternalServerError,
            AuthError::InternalError(_) => Status::InternalServerError,
        };

        let body = serde_json::to_string(&self).unwrap_or_else(|_| "{\"error\":\"Internal Server Error\"}".to_string());

        Response::build()
            .status(status_code)
            .header(ContentType::JSON)
            .sized_body(body.len(), std::io::Cursor::new(body))
            .ok()
    }
}

impl std::error::Error for AuthError {}

pub struct JwtAuth {
    pub secret: Arc<String>,
    pub algorithm: Algorithm,
    pub validation: Validation,
}
