use std::io::Cursor;

use async_compression::tokio::bufread::{BrotliEncoder, DeflateEncoder, GzipEncoder};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::hyper::header::{ACCEPT_ENCODING, CONTENT_ENCODING};
use rocket::http::{ContentType, Header};
use rocket::{Request, Response};
use tokio::io::{AsyncReadExt, BufReader};

pub struct Compression;

#[rocket::async_trait]
impl Fairing for Compression {
    fn info(&self) -> Info {
        Info {
            name: "Compression",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let headers = request.headers();
        if let Some(accept_encoding) = headers.get_one(ACCEPT_ENCODING.as_str()) {
            if let Some(content_type) = response.content_type() {
                if content_type == ContentType::JSON
                    || content_type == ContentType::Plain
                    || content_type == ContentType::HTML
                {
                    if let Ok(body_bytes) = response.body_mut().to_bytes().await {
                        if accept_encoding.contains("br") {
                            let mut encoded = Vec::new();
                            let mut encoder = BrotliEncoder::new(BufReader::new(Cursor::new(body_bytes)));
                            if encoder.read_to_end(&mut encoded).await.is_ok() {
                                response.set_header(Header::new(CONTENT_ENCODING.as_str(), "br"));
                                response.set_sized_body(encoded.len(), Cursor::new(encoded));
                                return;
                            }
                        } else if accept_encoding.contains("gzip") {
                            let mut encoded = Vec::new();
                            let mut encoder = GzipEncoder::new(BufReader::new(Cursor::new(body_bytes)));
                            if encoder.read_to_end(&mut encoded).await.is_ok() {
                                response.set_header(Header::new(CONTENT_ENCODING.as_str(), "gzip"));
                                response.set_sized_body(encoded.len(), Cursor::new(encoded));
                                return;
                            }
                        } else if accept_encoding.contains("deflate") {
                            let mut encoded = Vec::new();
                            let mut encoder = DeflateEncoder::new(BufReader::new(Cursor::new(body_bytes)));
                            if encoder.read_to_end(&mut encoded).await.is_ok() {
                                response.set_header(Header::new(CONTENT_ENCODING.as_str(), "deflate"));
                                response.set_sized_body(encoded.len(), Cursor::new(encoded));
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
