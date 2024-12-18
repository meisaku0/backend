use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;
use sea_orm::prelude::Uuid;

#[derive(schemars::JsonSchema, Serialize, Deserialize, Validate, Clone)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct ActiveEmailDTO {
    /// The token to activate the email.
    ///
    /// This token is generated by the server and sent to the user's email.
    pub token: Uuid,
    /// User id.
    ///
    /// The id of the user that the email is being activated for.
    pub user_id: Uuid,
}
