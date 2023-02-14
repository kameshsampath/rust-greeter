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
    }

    async fn root() -> (StatusCode, String) {
        if let Ok(message) = env::var("DEFAULT_MESSAGE") {
            return (StatusCode::OK, message);
        } else {
            return (StatusCode::OK, "Hello World!".to_string());
        }
    }
}
