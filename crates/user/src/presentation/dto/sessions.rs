use rocket::serde::{Deserialize, Serialize};
use rocket::FromForm;
use rocket_validation::Validate;

/// # User Session Paginate DTO
///
/// ## Example
///
/// ```json
/// {
///     "page": 1,
///     "per_page": 10,
///     "ip": "1.1.1.1",
///     "browser": "Chrome",
///     "os": "Windows"
/// }
#[derive(schemars::JsonSchema, Serialize, Deserialize, Validate, Clone, FromForm)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct UserSessionPaginateDTO {
    /// The page number.
    #[schemars(length(min = 1, max = 99))]
    #[validate(range(min = 1, max = 99))]
    pub page: u64,

    /// The number of items per page.
    #[schemars(length(min = 1, max = 99))]
    #[validate(range(min = 1, max = 99))]
    pub per_page: u64,

    /// The IP address.
    pub ip: Option<String>,

    /// The browser.
    pub browser: Option<String>,

    /// The operating system.
    pub device: Option<String>,

    /// The operating system.
    pub os: Option<String>,
}
