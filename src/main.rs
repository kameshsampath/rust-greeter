use std::{env, net::SocketAddr};

use rgreeter::service::HelloWorld as hello_world_service;
use tracing::info;
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>()
        .unwrap();

    // initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(hello_world_service::new().service())
        .await
        .unwrap();
}
