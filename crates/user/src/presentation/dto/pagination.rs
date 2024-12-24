use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

use crate::domain::entities::UserSessionEntity::SessionMinimal;

/// # Item Pagination DTO
///
/// The data transfer object for paginating items.
#[derive(schemars::JsonSchema, Serialize, Deserialize, Validate, Clone)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct ItemPaginationDTO {
    /// The data of the sessions.
    pub items: Vec<SessionMinimal>,

    /// The total number of items.
    pub total_items: u64,

    /// The number of pages.
    pub total_pages: u64,

    /// The current page.
    pub page: u64,

    /// The number of items per page.
    pub per_page: u64,

    /// Has next page.
    pub has_next_page: bool,

    /// Has previous page.
    pub has_previous_page: bool,
}
