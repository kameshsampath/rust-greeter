use std::collections::HashMap;

use axum::{extract::Query, http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Greeting {
    pub message: String,
}

pub async fn greet(Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<Greeting>) {
    let mut greeting = Greeting {
        message: "Hello! Anonymous!".to_string(),
    };

    if let Some(name) = params.get("name") {
        greeting.message = format!("Hello!{}", name);
    }

    (StatusCode::OK, Json(greeting))
}
