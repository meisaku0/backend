use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

/// # Change Username DTO
///
/// This DTO is used to change the username of a user.
#[derive(schemars::JsonSchema, Serialize, Deserialize, Validate, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ChangeUsernameDTO {
    #[schemars(length(min = 3, max = 16))]
    #[validate(length(min = 3, max = 16, message = "Username must be between 3 and 16 characters long."))]
    /// The new username.
    pub username: String,
}
