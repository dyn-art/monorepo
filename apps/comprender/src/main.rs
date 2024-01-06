use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{routing::get, Router};
// use rspc::{BuiltRouter, Rspc};

// const R: Rspc<()> = Rspc::new();

// fn router() -> Arc<BuiltRouter> {
//     R.router()
//         .procedure("version", R.query(|ctx, _: ()| env!("CARGO_PKG_VERSION")))
//         .build()
//         .unwrap()
//         .arced()
// }

#[tokio::main]
async fn main() {
    // let router = router();
    // router
    //     .export_ts(ExportConfig::new(
    //         PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../bindings.ts"),
    //     ))
    //     .unwrap();

    // Build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    // .route("/rspc/:id", router.endpoint(|| ()).axum());

    // Run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
