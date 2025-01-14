use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

/// # Change Password DTO
///
/// The data transfer object for changing a user's password.
#[derive(schemars::JsonSchema, Serialize, Deserialize, Validate, Clone)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct ChangePasswordDTO {
    #[schemars(length(min = 6, max = 32))]
    #[validate(length(min = 6, max = 32, message = "Password must be between 6 and 32 characters long."))]
    /// The user's current password.
    pub current_password: String,
    #[schemars(length(min = 6, max = 32))]
    #[validate(length(min = 6, max = 32, message = "Password must be between 6 and 32 characters long."))]
    pub new_password: String,
}
