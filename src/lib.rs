pub mod service {
    use std::env;

    use axum::{http::StatusCode, routing::get, routing::IntoMakeService, Router};

    pub mod greeter_routes;

    pub struct HelloWorld(Router);

    impl HelloWorld {
        pub fn new() -> Self {
            let app = Router::new()
                .route("/", get(root))
                .route("/greet", get(greeter_routes::greet));
            HelloWorld(app)
        }

        pub fn service(self) -> IntoMakeService<Router> {
            self.0.into_make_service()
        }

        pub fn app(self) -> Router {
            self.0
        }
    }

    async fn root() -> (StatusCode, String) {
        if let Ok(message) = env::var("DEFAULT_MESSAGE") {
            return (StatusCode::OK, message);
        } else {
            return (StatusCode::OK, "Hello World!".to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::service::HelloWorld as hello_world_service;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[tokio::test]
    async fn get_root() {
        let app = hello_world_service::new().app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();

        let body_str = std::str::from_utf8(&body_bytes).unwrap();

        assert_eq!(
            body_str, "Hello World!",
            "Expecting `Hello World!` but got {}",
            body_str
        );
    }

    #[tokio::test]
    async fn default_greeting() {
        let app = hello_world_service::new().app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/greet")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

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

    #[tokio::test]
    async fn greeting_by_name() {
        let app = hello_world_service::new().app();
        let uri = format!("/greet?name={}", "Jack");
        let response = app
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();

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

    #[tokio::test]
    async fn not_found_uri() {
        let app = hello_world_service::new().app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/not-found")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

        assert!(
            body.is_empty(),
            "Expecting body to be empty but got {}",
            std::str::from_utf8(&body).unwrap()
        );
    }
}
