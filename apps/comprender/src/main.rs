use axum::{routing::get, Json, Router, Server};
use serde::Serialize;
use std::os::unix::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let address = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {address}");
    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello World"),
    })
}
