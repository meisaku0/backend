use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{error_, Request, Response};
use time::format_description::well_known::Rfc2822;
use time::{Duration, OffsetDateTime};

pub struct CacheControl {
    max_age: Option<Duration>,
    public: bool,
    private: bool,
    no_cache: bool,
    no_store: bool,
    must_revalidate: bool,
    expires: Option<OffsetDateTime>,
}

impl Default for CacheControl {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheControl {
    pub fn new() -> Self {
        CacheControl {
            max_age: None,
            public: false,
            private: false,
            no_cache: false,
            no_store: false,
            must_revalidate: false,
            expires: None,
        }
    }

    pub fn max_age(mut self, duration: Duration) -> Self {
        self.max_age = Some(duration);
        self
    }

    pub fn public(mut self) -> Self {
        self.public = true;
        self
    }

    pub fn private(mut self) -> Self {
        self.private = true;
        self
    }

    pub fn no_cache(mut self) -> Self {
        self.no_cache = true;
        self
    }

    pub fn no_store(mut self) -> Self {
        self.no_store = true;
        self
    }

    pub fn must_revalidate(mut self) -> Self {
        self.must_revalidate = true;
        self
    }

    pub fn expires(mut self, time: OffsetDateTime) -> Self {
        self.expires = Some(time);
        self
    }
}

#[rocket::async_trait]
impl Fairing for CacheControl {
    fn info(&self) -> Info {
        Info {
            name: "Cache Control",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        let mut cache_control_directives = Vec::new();

        if let Some(max_age) = self.max_age {
            cache_control_directives.push(format!("max-age={}", max_age.whole_seconds()));
        }

        if self.public {
            cache_control_directives.push("public".to_string());
        }

        if self.private {
            cache_control_directives.push("private".to_string());
        }

        if self.no_cache {
            cache_control_directives.push("no-cache".to_string());
        }

        if self.no_store {
            cache_control_directives.push("no-store".to_string());
        }

        if self.must_revalidate {
            cache_control_directives.push("must-revalidate".to_string());
        }

        if !cache_control_directives.is_empty() {
            let cache_control_value = cache_control_directives.join(", ");
            response.set_header(Header::new("Cache-Control", cache_control_value));
        }

        if let Some(expires) = self.expires {
            match expires.format(&Rfc2822) {
                Ok(expires_str) => {
                    response.set_header(Header::new("Expires", expires_str));
                }
                Err(e) => {
                    error_!("Failed to set expiration date: {}", e);
                }
            }
        }
    }
}
