use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

#[derive(schemars::JsonSchema, Serialize, Deserialize, Validate, Clone, Debug)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct RevokeSessionDTO {
    /// The session ID to revoke.
    ///
    /// If not provided, all sessions will be revoked.
    pub session_id: Option<String>,
}
