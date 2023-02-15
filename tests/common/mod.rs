use std::env;

use axum::{body::Body, http::Error};
use hyper::Request;

pub fn build_request_with_no_body(uri_path: &str) -> Result<Request<Body>, Error> {
    let service_url = env::var("SERVICE_URL")
        .expect("Environment variable `SERVICE_URL` set to service url should be set.");
    let uri = format!("{}{}", service_url, uri_path);
    Request::builder().uri(&uri).body(Body::empty())
}
