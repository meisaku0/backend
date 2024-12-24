use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

use crate::domain::entities::email::PartialEmail;
use crate::domain::entities::UserEntity::PartialUser;

/// # User me
///
/// Get the current user data
#[derive(schemars::JsonSchema, Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct UserMeDTO {
    /// User data (partial)
    pub user: Option<PartialUser>,

    /// Email data (partial)
    pub email: Option<PartialEmail>,
}
