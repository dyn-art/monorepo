use dyn_comprender::{app, environment::app_config::AppConfig};
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    print!("\n\n\n");
    println!("⏳ Starting server..");

    // Load config from environment
    let app_config = match AppConfig::from_env() {
        Ok(env) => {
            println!("🟩 Loaded environment: {:?}", env);
            env
        }
        Err(_) => {
            println!("🟥 Failed to load required environment variables!");
            return;
        }
    };

    // Run app with hyper, listening globally on specified port
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, app_config.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let app = app::setup(app_config.clone());
    println!("🚀 Starting server v{}", app_config.pkg_version);
    println!("📡 Awaiting requests");
    println!(
        "👂 Listening on address: {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
