use rocket::serde::{Deserialize, Serialize};

pub mod database;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
    pub resend_api_key: Option<String>,
    pub resend_from_email: Option<String>,
    pub jwt_secret: Option<String>,
}
