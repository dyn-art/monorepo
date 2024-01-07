use std::{
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};

use axum::{routing::get, Router};
use rspc::{BuiltRouter, ExportConfig, Rspc};

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
    let router = router();

    // Only export types in development builds
    #[cfg(debug_assertions)]
    router
        .export_ts(ExportConfig::new(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../packages/types/src/comprender/gen/bindings.ts"),
        ))
        .unwrap();

    // Build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async { "dyn_comprender is up and running! Connect to RSPC via '/rspc'." }),
        )
        .nest("/rspc", router.endpoint(|| ()).axum());

    // Run our app with hyper, listening globally on port 4000
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 4000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
