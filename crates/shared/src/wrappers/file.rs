use rocket::form::{DataField, FromFormField, Options, ValueField};
use rocket::fs::TempFile;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::gen::SchemaGenerator;
use rocket_okapi::okapi::schemars::schema::Schema;

/// This is a wrapper for the rocket.rs TempFile that is not implemented in the
/// version of schemars that rocket_okapi uses.
///
/// This is a temporary solution until the rocket_okapi crate is updated to
/// support TempFile.
#[derive(Debug)]
pub struct TempFileWrapper<'f>(pub TempFile<'f>);

impl<'f> schemars::JsonSchema for TempFileWrapper<'f> {
    fn schema_name() -> String { "Temp uploaded file".to_string() }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema { <Vec<u8>>::json_schema(gen) }
}

#[rocket::async_trait]
impl<'f> rocket::form::FromForm<'f> for TempFileWrapper<'f> {
    type Context = Option<TempFile<'f>>;

    fn init(_opts: Options) -> Self::Context { None }

    fn push_value(_: &mut Self::Context, _field: ValueField<'f>) {}

    async fn push_data(ctx: &mut Self::Context, field: DataField<'f, '_>) {
        if ctx.is_none() {
            if let Ok(temp_file) = TempFile::from_data(field).await {
                *ctx = Some(temp_file);
            }
        }
    }

    fn finalize(ctx: Self::Context) -> rocket::form::Result<'f, Self> {
        match ctx {
            Some(temp_file) => Ok(TempFileWrapper(temp_file)),
            None => Err(rocket::form::Errors::from(rocket::form::Error::validation("missing file field"))),
        }
    }
}
