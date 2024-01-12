use std::net::{Ipv4Addr, SocketAddr};

use crate::environment::config::Config;

mod app;
mod environment;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let config = Config::new().expect("Failed to load configuration");

    // Run app with hyper, listening globally on specified port
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, config.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!(
        "🚀 Server (v{}) started successfully and is listening on port: {}",
        env!("CARGO_PKG_VERSION"),
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app::app()).await.unwrap();
}
