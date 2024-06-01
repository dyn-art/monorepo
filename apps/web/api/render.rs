use bevy_app::App;
use bevy_ecs::query::{With, Without};
use dyn_comp_asset::asset::AssetContent;
use dyn_comp_bundles::components::marker::Root;
use dyn_comp_core::{resources::composition::CompositionRes, CompCorePlugin};
use dyn_comp_dtif::DtifComposition;
use dyn_comp_svg_builder::{svg::svg_bundle::SvgBundleVariant, CompSvgBuilderPlugin};
use dyn_web_api::{
    app_error,
    models::app_error::{AppError, ErrorCode, IntoVercelResponse},
};
use std::collections::HashMap;
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let url = match Url::parse(&_req.uri().to_string()) {
        Ok(v) => v,
        Err(_) => {
            return app_error!(StatusCode::BAD_REQUEST, ErrorCode::new("INVLAID_URL"))
                .into_vercel_response()
        }
    };

    // Parse query params
    let query_params = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
    let format = query_params
        .get("format")
        .map(|s| s.as_str())
        .unwrap_or("svg");

    // Parse body
    let mut body: DtifComposition = match serde_json::from_slice(_req.body()) {
        Ok(v) => v,
        Err(_) => {
            return app_error!(StatusCode::BAD_REQUEST, ErrorCode::new("INVALID_BODY"))
                .into_vercel_response()
        }
    };

    // Build SVG string
    match prepare_dtif_composition(&mut body).await {
        Err(_) => {
            return app_error!(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorCode::new("FAILED_TO_FETCH_RESOURCES")
            )
            .into_vercel_response()
        }
        _ => {}
    };
    let svg_string = match build_svg_string(body) {
        Ok(v) => v,
        Err(err) => return err.into_vercel_response(),
    };

    // Determine and parse SVG to response format specified by format query parameter
    return match format {
        "png" => {
            let fontdb = fontdb::Database::new();
            let opts = usvg::Options::default();

            let tree = usvg::Tree::from_str(&svg_string, &opts, &fontdb).unwrap();

            let pixmap_size = tree.size().to_int_size();
            let mut pixmap =
                tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
            resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
            let png_data = pixmap.encode_png().unwrap();

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/png")
                .body(Body::from(png_data))
                .unwrap())
        }
        "svg" => {
            let opts = usvg::Options::default();
            let fontdb = fontdb::Database::new();

            let tree = usvg::Tree::from_str(&svg_string, &opts, &fontdb).unwrap();

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/svg+xml")
                .body(Body::from(
                    tree.to_string(&usvg::WriteOptions::default()).into_bytes(),
                ))
                .unwrap())
        }
        "csvg" => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "image/svg+xml")
            .body(Body::from(svg_string.into_bytes()))
            .unwrap()),
        "pdf" => {
            let pdf_data = svg2pdf::convert_str(&svg_string, svg2pdf::Options::default()).unwrap();

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/pdf")
                .body(Body::from(pdf_data))
                .unwrap())
        }
        _ => app_error!(
            StatusCode::BAD_REQUEST,
            ErrorCode::new("INVALID_FORMAT"),
            "Supported formats are: 'png', 'svg', 'csvg' and 'pdf'"
        )
        .into_vercel_response(),
    };
}

async fn prepare_dtif_composition(
    dtif_composition: &mut DtifComposition,
) -> Result<(), reqwest::Error> {
    for asset in dtif_composition.assets.iter_mut() {
        let mut maybe_content = None;
        if let AssetContent::Url { url } = &asset.content {
            maybe_content = Some(reqwest::get(url).await?.bytes().await?.to_vec());
        }
        if let Some(content) = maybe_content {
            asset.content = AssetContent::Binary { content }
        }
    }

    return Ok(());
}

fn build_svg_string(mut dtif: DtifComposition) -> Result<String, AppError> {
    let mut app = App::new();

    // Register plugins
    app.add_plugins((
        CompCorePlugin {
            version: dtif.version,
            size: dtif.size,
            viewport: dtif.viewport,
        },
        CompSvgBuilderPlugin {},
    ));

    dtif.send_into_world(&mut app.world);

    // Update app once
    app.update();

    let mut result = String::new();
    let comp_res = app
        .world
        .get_resource::<CompositionRes>()
        .ok_or(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::new("COMPOSITION_RES_NOT_FOUND"),
        ))?;

    // Open SVG tag
    result.push_str(&format!(
        "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
        comp_res.size.width(),
        comp_res.size.height()
    ));

    let mut system_state: bevy_ecs::system::SystemState<(
        bevy_ecs::system::Query<&SvgBundleVariant, With<Root>>,
        bevy_ecs::system::Query<&SvgBundleVariant, Without<Root>>,
    )> = bevy_ecs::system::SystemState::new(&mut app.world);
    let (root_bundle_variant_query, bundle_variant_query) = system_state.get(&mut app.world);

    // Construct SVG string starting from root nodes
    root_bundle_variant_query.iter().for_each(|bundle_variant| {
        result.push_str(&bundle_variant.to_string(&bundle_variant_query))
    });

    // Close the SVG tag
    result.push_str("</svg>");

    return Ok(result);
}
