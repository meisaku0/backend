use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{info_, Data, Request, Response};
use uuid::Uuid;

pub struct RequestId;

#[derive(Clone)]
struct Id(Option<String>);

#[rocket::async_trait]
impl Fairing for RequestId {
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Response | Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        if let Some(request_id) = request.headers().get_one("x-request-id") {
            request.local_cache(|| Id(Some(request_id.to_string())));
            return;
        }

        let request_id = request
            .headers()
            .get_one("x-request-id")
            .map(ToString::to_string)
            .or_else(|| Some(Uuid::new_v4().to_string()));

        request.local_cache(|| Id(request_id));
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let request_id = request.local_cache(|| Id(None));
        let request_id = request_id.clone().0.unwrap_or("unknown".to_string());

        info_!("Request ID: {:?}", request_id);

        response.set_header(Header::new("x-request-id", request_id));
    }
}
