use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

/// Data transfer object for user sign in
#[derive(schemars::JsonSchema, Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct CredentialsDTO {
    /// Email of the user
    ///
    /// Must be a valid email address format.
    #[schemars(email)]
    pub email: String,

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
    pub refresh_token: String,

    /// Expiration time of the access token in seconds
    pub expires_in: i64,

    /// Type of the token
    pub token_type: String,
}
