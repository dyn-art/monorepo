use std::net::{Ipv4Addr, SocketAddr};

use config::Config;

use crate::app::app;

mod app;
mod config;

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
    axum::serve(listener, app()).await.unwrap();
}
