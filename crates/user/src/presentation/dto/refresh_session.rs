use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

/// # Refresh Session
///
/// This is the data that is required to refresh a user's session.
#[derive(schemars::JsonSchema, Serialize, Deserialize, Validate, Clone)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct RefreshSessionDTO {
    /// The refresh token
    ///
    /// This is the refresh token that was returned when the user logged in.
    pub refresh_token: String,
}
