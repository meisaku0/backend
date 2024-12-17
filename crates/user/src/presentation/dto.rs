use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

/// # Create user
///
/// Data transfer object for creating a user.
#[derive(schemars::JsonSchema, Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct CreateUserDTO {
    /// The username of the user. Must be unique.
    #[schemars(length(min = 3, max = 16))]
    #[validate(length(min = 3, max = 16, message = "Username must be between 3 and 16 characters long."))]
    pub username: String,
    /// The email of the user. Must be unique.
    #[schemars(email)]
    #[validate(email(message = "Email must be a valid email address."))]
    pub email: String,
    /// The password of the user.
    #[schemars(length(min = 6, max = 32))]
    #[validate(length(min = 6, max = 32, message = "Password must be between 6 and 32 characters long."))]
    pub password: String,
}
