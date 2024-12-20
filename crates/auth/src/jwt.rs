use std::collections::HashSet;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rocket::serde::{Deserialize, Serialize};
use shared::responses::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub scopes: HashSet<String>,
}

pub struct JwtAuth {
    secret: Arc<String>,
    algorithm: Algorithm,
    validation: Validation,
}

impl JwtAuth {
    pub fn new(secret: String) -> Self {
        JwtAuth {
            secret: Arc::new(secret),
            algorithm: Algorithm::HS256,
            validation: Validation::new(Algorithm::HS256),
        }
    }

    pub fn generate_token(&self, sub: String, scopes: HashSet<String>, ttl: u64) -> Result<String, AppError> {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::InvalidTime)?
            .as_secs()
            + ttl;

        let claims = Claims {
            sub,
            exp,
            scopes,
        };
        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_ref().as_bytes()))
            .map_err(|e| AppError::TokenCreationError(e.to_string()))
    }

    pub fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, AppError> {
        let data =
            decode::<Claims>(token, &DecodingKey::from_secret(self.secret.as_ref().as_bytes()), &self.validation)
                .map_err(|e| AppError::TokenValidationError(e.to_string()))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::InvalidTime)?
            .as_secs();

        if data.claims.exp < now {
            return Err(AppError::ExpiredToken);
        }

        Ok(data)
    }

    pub fn validate_scope(&self, token: &str, required_scope: &str) -> Result<(), AppError> {
        let data = self.validate_token(token)?;
        if !data.claims.scopes.contains(required_scope) {
            return Err(AppError::MissingScope(required_scope.to_string()));
        }
        Ok(())
    }

    pub fn validate_multiple_scopes(&self, token: &str, required_scopes: &[&str]) -> Result<(), AppError> {
        let data = self.validate_token(token)?;
        let missing_scopes: Vec<&str> = required_scopes
            .iter()
            .filter(|&&scope| !data.claims.scopes.contains(scope))
            .cloned()
            .collect();

        if !missing_scopes.is_empty() {
            return Err(AppError::MissingScope(format!("Missing required scopes: {:?}", missing_scopes)));
        }

        Ok(())
    }
}
