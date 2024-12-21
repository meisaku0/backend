use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;
use sea_orm::prelude::Uuid;

/// Data transfer object for user sign in
#[derive(schemars::JsonSchema, Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct CredentialsDTO {
    /// Username of the user
    ///
    /// Must be a string.
    pub username: String,

    /// Password of the user
    ///
    /// Must be between 6 and 32 characters long.
    #[schemars(length(min = 6, max = 32))]
    #[validate(length(min = 6, max = 32, message = "Password must be between 6 and 32 characters long."))]
    pub password: String,
}

/// Data transfer object for user sign in response
#[derive(schemars::JsonSchema, Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct SignInDTO {
    /// Generated JWT access token for the user session
    pub access_token: String,

    /// Generated JWT refresh token for the user session
    pub refresh_token: Option<String>,

    /// Expiration time of the access token in seconds
    pub expires_in: u64,

    /// Type of the token
    pub token_type: String,

    /// Username of the user
    pub username: String,

    /// User ID
    pub user_id: Uuid,
}
