use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

/// # Reset Password
///
/// This is the data that is required to reset a user's password.
#[derive(schemars::JsonSchema, Serialize, Deserialize, Validate, Clone)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct ResetPasswordDTO {
    /// The reset token
    ///
    /// This is the reset token that was returned when the user requested a
    /// password reset.
    pub reset_token: String,
    /// The new password
    ///
    /// This is the new password that the user wants to set.
    pub new_password: String,
}
