use dyn_graphic::{
    app,
    environment::{app_config::AppConfig, logger::setup_logger},
};
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    setup_logger();

    log::info!("\n\n\n");
    log::info!("⏳ Starting server..");

    // Load config from environment
    let app_config = match AppConfig::from_env() {
        Ok(env) => {
            log::info!("🟩 Loaded environment: {:?}", env);
            env
        }
        Err(_) => {
            log::info!("🟥 Failed to load required environment variables!");
            return;
        }
    };

    // Run app with hyper, listening globally on specified port
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, app_config.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let app = app::setup(app_config.clone());
    log::info!("🚀 Starting server v{}", app_config.pkg_version);
    log::info!("📡 Awaiting requests");
    log::info!(
        "👂 Listening on address: {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
