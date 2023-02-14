use std::{env, net::SocketAddr};

use rust_hello_world::service::HelloWorld as hello_world_service;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(hello_world_service::new().service())
        .await
        .unwrap();
}
