use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    body::Body,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use config::Config;
use dyn_bevy_render_skeleton::RenderApp;
use dyn_composition::core::{composition::Composition, dtif::DTIFComposition};
use dyn_svg_render::{resources::svg_composition::SVGCompositionRes, SvgRenderPlugin};

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

// async fn handler() -> Result<String, String> {
//     // Initalize composition
//     let mut composition = Composition::new(Some(v));
//     let app = composition.get_app_mut();
//     // Register plugins
//     app.add_plugins(SvgRenderPlugin {
//         render_event_sender: None,
//     });
//     // Update app once
//     app.update();
//     // Get SVG string
//     let maybe_render_app = app.get_sub_app(RenderApp).ok();
//     if let Some(render_app) = maybe_render_app {
//         let maybe_svg_composition_res = render_app.world.get_resource::<SVGCompositionRes>();
//         if let Some(svg_composition_res) = maybe_svg_composition_res {
//             // return Some(svg_composition_res.to_string());
//         }
//     }

//     // let body = Body::from_stream(stream);

//     Ok(String::from("Hello World"))
// }
