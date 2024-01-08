use std::{
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};

use axum::{routing::get, Router};
use config::Config;
use rspc::{BuiltRouter, ExportConfig, Rspc};

mod config;

const R: Rspc<()> = Rspc::new();

fn router() -> Arc<BuiltRouter> {
    R.router()
        .procedure("version", R.query(|ctx, _: ()| env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap()
        .arced()
}

#[tokio::main]
async fn main() {
    let config = Config::new().expect("Failed to load configuration");
    let router = router();

    // Only export types in development builds
    #[cfg(debug_assertions)]
    router
        .export_ts(ExportConfig::new(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../packages/types/src/comprender/gen/bindings.ts"),
        ))
        .unwrap();

    // Build application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async { "dyn_comprender is up and running! Connect to RSPC via '/rspc'." }),
        )
        .nest("/rspc", router.endpoint(|| ()).axum());

    // Run app with hyper, listening globally on specified port
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, config.port));
    println!("Server starting on port: {}", config.port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
