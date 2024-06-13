use crate::{
    environment::app_state::AppState,
    error::{
        app_error::{AppError, ErrorCode},
        extract::{extract_json_body, extract_query_params, AppJson, AppQuery},
    },
};
use axum::{
    body::Body,
    extract::Query,
    http::{header, StatusCode},
    response::Response,
    Json,
};
use axum::{routing::post, Router};
use bevy_app::App;
use bevy_ecs::query::{With, Without};
use dyn_comp_asset::asset::AssetContent;
use dyn_comp_bundles::components::marker::Root;
use dyn_comp_core::{resources::composition::CompositionRes, CompCorePlugin};
use dyn_comp_dtif::DtifComposition;
use dyn_comp_svg_builder::{
    events::SvgBuilderOutputEvent, svg::svg_bundle::SvgBundleVariant, CompSvgBuilderPlugin,
};
use resvg::usvg::Options;
use serde::Deserialize;
use std::sync::mpsc::channel;
use usvg::WriteOptions;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(handler))
}

#[derive(Deserialize, utoipa::IntoParams)]
struct QueryParams {
    format: FileFormat,
}

#[derive(Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum FileFormat {
    Png,
    Svg,
    Pdf,
    RawSvg,
}

#[utoipa::path(
    post,
    path = "/v1/comp/render",
    responses(
        (status = 200, description = "Generation success", body = String),
        (status = BAD_REQUEST, description = "Bad Request", body = AppError)
    ),
    params(
        QueryParams,
    )
)]
#[axum::debug_handler]
async fn handler(
    maybe_query: AppQuery<QueryParams>,
    maybe_body: AppJson<DtifComposition>,
) -> Result<Response, AppError> {
    let params = extract_query_params(maybe_query)?;
    let mut body = extract_json_body(maybe_body)?;

    let _ = prepare_dtif_composition(&mut body).await;
    let svg_string = build_svg_string(body)?;

    // Determine response format from query parameter
    match params.format {
        FileFormat::Png => {
            let opts = Options::default();

            let tree = usvg::Tree::from_str(&svg_string, &opts).unwrap();

            let pixmap_size = tree.size().to_int_size();
            let mut pixmap =
                tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
            resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
            let png_data = pixmap.encode_png().unwrap();

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "image/png")
                .body(Body::from(png_data))
                .unwrap())
        }
        FileFormat::Svg => {
            let opts = Options::default();

            let tree = usvg::Tree::from_str(&svg_string, &opts).unwrap();

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "image/svg+xml")
                .body(Body::from(
                    tree.to_string(&WriteOptions::default()).into_bytes(),
                ))
                .unwrap())
        }
        FileFormat::RawSvg => Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "image/svg+xml")
            .body(Body::from(svg_string.into_bytes()))
            .unwrap()),
        FileFormat::Pdf => {
            let opts = Options::default();
            let tree = usvg::Tree::from_str(&svg_string, &opts).unwrap();
            let pdf_data = svg2pdf::to_pdf(
                &tree,
                svg2pdf::ConversionOptions::default(),
                svg2pdf::PageOptions::default(),
            );

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/pdf")
                .body(Body::from(pdf_data))
                .unwrap())
        }
    }
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

    let (svg_builder_output_event_sender, _) = channel::<SvgBuilderOutputEvent>();

    // Register plugins
    app.add_plugins((
        CompCorePlugin {
            version: dtif.version,
            size: dtif.size,
            viewport: dtif.viewport,
        },
        CompSvgBuilderPlugin {
            output_event_sender: svg_builder_output_event_sender,
        },
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