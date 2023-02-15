use axum::http::StatusCode;
use serde_json::{json, Value};

pub mod common;

#[tokio::test]
async fn it_get_root() {
    let client = hyper::Client::new();
    match common::build_request_with_no_body("/") {
        Ok(req) => {
            let response = client.request(req).await.unwrap();

            assert_eq!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            let body = std::str::from_utf8(&body).unwrap();

            assert_eq!(
                body, "Hello World!",
                "Expecting `Hello World!` but got {}",
                body
            );
        }
        Err(e) => panic!("{}", e),
    }
}

#[tokio::test]
async fn it_default_greeting() {
    let client = hyper::Client::new();
    match common::build_request_with_no_body("/greet") {
        Ok(req) => {
            let response = client.request(req).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

            let body: Value = serde_json::from_slice(&body).unwrap();

            assert_eq!(
                body,
                json!({"message": "Hello! Anonymous!"}),
                "Expecting `{{\"message\": \"Hello! Anonymous!\"}}` but got {:?}",
                body
            );
        }
        Err(e) => panic!("{}", e),
    }
}

#[tokio::test]
async fn it_greeting_by_name() {
    let client = hyper::Client::new();
    match common::build_request_with_no_body(format!("/greet?name={}", "Jack").as_str()) {
        Ok(req) => {
            let response = client.request(req).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);

            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

            let body: Value = serde_json::from_slice(&body).unwrap();

            assert_eq!(
                body,
                json!({"message": "Hello! Jack!"}),
                "Expecting `{{\"message\": \"Hello! Jack!\"}}` but got {:?}",
                body
            );
        }
        Err(e) => panic!("{}", e),
    }
}

#[tokio::test]
async fn it_not_found_uri() {
    let client = hyper::Client::new();
    match common::build_request_with_no_body("/not-found") {
        Ok(req) => {
            let response = client.request(req).await.unwrap();
            assert_eq!(response.status(), StatusCode::NOT_FOUND);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert!(
                body.is_empty(),
                "Expecting body to be empty but got {}",
                std::str::from_utf8(&body).unwrap()
            );
        }
        Err(e) => panic!("{}", e),
    }
}
