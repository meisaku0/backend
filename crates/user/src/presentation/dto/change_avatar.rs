use rocket::FromForm;
use rocket_validation::Validate;
use shared::wrappers::file::TempFileWrapper;

/// Change avatar DTO
///
/// This DTO is used to change the avatar of a user.
#[derive(schemars::JsonSchema, Validate, FromForm)]
#[serde(crate = "rocket::serde")]
#[schemars(deny_unknown_fields)]
pub struct ChangeAvatar<'a> {
    /// The avatar of the user.
    pub avatar: TempFileWrapper<'a>,
}
