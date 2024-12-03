use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header};

pub struct Helmet;

#[rocket::async_trait]
impl Fairing for Helmet {
    fn info(&self) -> Info {
        Info {
            name: "Helmet (Headers and Content Security Policy)",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Content-Security-Policy", "default-src 'self';base-uri 'self';font-src 'self' https: data:;form-action 'self';frame-ancestors 'self';img-src 'self' data:;object-src 'none';script-src 'self';script-src-attr 'none';style-src 'self' https: 'unsafe-inline';upgrade-insecure-requests"));
        response.set_header(Header::new("Cross-Origin-Embedder-Policy", "require-corp"));
        response.set_header(Header::new("Cross-Origin-Opener-Policy", "same-origin"));
        response.set_header(Header::new("Cross-Origin-Resource-Policy", "same-site"));
        response.set_header(Header::new("Origin-Agent-Cluster", "?1"));
        response.set_header(Header::new("X-Download-Options", "noopen"));
        response.set_header(Header::new("X-Permitted-Cross-Domain-Policies", "none"));
    }
}