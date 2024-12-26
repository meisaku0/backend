use handlebars::{DirectorySourceOptions, Handlebars};
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::Resend;

pub struct ResendMailer {
    pub client: Resend,
    pub from_email: String,
    pub templates: Handlebars<'static>,
}

impl ResendMailer {
    pub fn new(api_key: String, from_email: String) -> Self {
        Self {
            client: Resend::new(&api_key),
            from_email,
            templates: Handlebars::new(),
        }
    }

    pub fn load_templates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.templates
            .register_templates_directory("templates/", DirectorySourceOptions::default())?;

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
