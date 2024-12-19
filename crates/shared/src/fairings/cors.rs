use std::collections::HashSet;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method};
use rocket::{info_, Request, Response};

#[derive(Default)]
pub struct Cors {
    allowed_origins: HashSet<String>,
    allowed_methods: HashSet<Method>,
    allowed_headers: HashSet<String>,
    allow_credentials: bool,
    expose_headers: HashSet<String>,
    max_age: Option<usize>,
}

impl Cors {
    pub fn new() -> Self { Self::default() }

    pub fn allowed_origins(mut self, origins: HashSet<String>) -> Self {
        self.allowed_origins = origins;
        self
    }

    pub fn allowed_methods(mut self, methods: HashSet<Method>) -> Self {
        self.allowed_methods = methods;
        self
    }

    pub fn allowed_headers(mut self, headers: HashSet<String>) -> Self {
        self.allowed_headers = headers;
        self
    }

    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    }

    pub fn expose_headers(mut self, headers: HashSet<String>) -> Self {
        self.expose_headers = headers;
        self
    }

    pub fn max_age(mut self, max_age: usize) -> Self {
        self.max_age = Some(max_age);
        self
    }

    pub fn build(self) -> Self { self }
}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response | Kind::Request,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if let Some(origin) = request.headers().get_one("Origin") {
            if self.allowed_origins.contains(origin) || self.allowed_origins.contains("*") {
                info_!("CORS request: {}", origin);

                response.set_header(Header::new("Access-Control-Allow-Origin", origin));

                if request.method() == Method::Options {
                    self.handle_preflight(request, response);
                } else {
                    self.add_cors_headers(response);
                }
            }
        }
    }
}

impl Cors {
    fn handle_preflight<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let methods = self
            .allowed_methods
            .iter()
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        response.set_header(Header::new("Access-Control-Allow-Methods", methods));

        if let Some(req_headers) = request.headers().get_one("Access-Control-Request-Headers") {
            response.set_header(Header::new("Access-Control-Allow-Headers", req_headers));
        }

        if let Some(max_age) = self.max_age {
            response.set_header(Header::new("Access-Control-Max-Age", max_age.to_string()));
        }
    }

    fn add_cors_headers(&self, response: &mut Response<'_>) {
        if self.allow_credentials {
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if !self.expose_headers.is_empty() {
            let headers = self.expose_headers.iter().cloned().collect::<Vec<_>>().join(", ");
            response.set_header(Header::new("Access-Control-Expose-Headers", headers));
        }
    }
}
