use std::sync::Arc;

use handlebars::Handlebars;
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::Resend;

#[derive(Clone)]
pub struct ResendMailer {
    pub client: Arc<Resend>,
    pub from_email: String,
    pub templates: Arc<Handlebars<'static>>,
}

impl ResendMailer {
    pub fn new(api_key: String, from_email: String) -> Self {
        Self {
            client: Arc::new(Resend::new(&api_key)),
            from_email,
            templates: Arc::new(Handlebars::new()),
        }
    }

    pub fn load_templates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let handlebars = Arc::get_mut(&mut self.templates).ok_or("Failed to get mutable reference to templates")?;
        handlebars.register_templates_directory("assets_email/templates ", Default::default())?;

        Ok(())
    }

    pub async fn send_email(
        &self, to_email: Vec<&str>, subject: &str, template_name: &str, data: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let html = self.templates.render(template_name, &data).unwrap();
        let options = CreateEmailBaseOptions::new(&self.from_email, to_email, subject).with_html(&html);

        self.client.emails.send(options).await?;

        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for ResendMailer {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        match request.guard::<&rocket::State<ResendMailer>>().await {
            rocket::request::Outcome::Success(state) => rocket::request::Outcome::Success(state.inner().clone()),
            rocket::request::Outcome::Error(_) => {
                rocket::request::Outcome::Error((rocket::http::Status::InternalServerError, ()))
            },
            rocket::request::Outcome::Forward(_) => {
                rocket::request::Outcome::Forward(rocket::http::Status::InternalServerError)
            },
        }
    }
}

impl rocket_okapi::request::OpenApiFromRequest<'_> for ResendMailer {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator, _name: String, _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        Ok(rocket_okapi::request::RequestHeaderInput::None)
    }
}
