use std::collections::HashMap;

use axum::{extract::Query, http::StatusCode, Json};
use serde::Serialize;

// Greeting holds the message that will be returned as part of the greet request response
#[derive(Serialize)]
pub struct Greeting {
    /// the greeting message e.g. Hello! World!
    pub message: String,
}
/// greet greets the user by Query Param `name`.
/// If not `name` is provided it returns `Hello! Anonymous!"`
pub async fn greet(Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<Greeting>) {
    let mut greeting = Greeting {
        message: "Hello! Anonymous!".to_string(),
    };

    if let Some(name) = params.get("name") {
        greeting.message = format!("Hello! {}!", name);
    }

    (StatusCode::OK, Json(greeting))
}

#[cfg(test)]
mod tests {

    use super::*;
    use axum::extract::Query;
    use std::collections::HashMap;

    #[tokio::test]
    async fn default_greeting() {
        let expected_params: HashMap<String, String> = HashMap::new();
        let query = Query(expected_params);
        let (status_code, json_greeting) = greet(query).await;
        assert_eq!(
            status_code.as_u16(),
            200,
            "Expecting HTTP OK but got {}",
            status_code.as_str()
        );
        let message = json_greeting.message.as_str();
        assert_eq!(
            message, "Hello! Anonymous!",
            "Expected `Hello! Anonymous!` but got {}",
            message
        );
    }
    #[tokio::test]
    async fn named_greeting() {
        let mut expected_params: HashMap<String, String> = HashMap::new();
        expected_params.insert("name".to_string(), "Jack".to_string());
        let query = Query(expected_params);
        let (status_code, json_greeting) = greet(query).await;
        assert_eq!(
            status_code.as_u16(),
            200,
            "Expecting HTTP OK but got {}",
            status_code.as_str()
        );
        let message = json_greeting.message.as_str();
        assert_eq!(
            message, "Hello! Jack!",
            "Expected `Hello! Jack!` but got {}",
            message
        );
    }
}
