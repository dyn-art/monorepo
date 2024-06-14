use crate::{
    environment::app_state::AppState,
    error::app_error::{AppError, AppErrorOptions, ErrorCode},
    middlewares::extract::{AppJson, AppQuery},
};
use axum::{
    body::Body,
    http::{header, StatusCode},
    response::Response,
};
use axum::{routing::post, Router};
use bevy_app::App;
use bevy_ecs::query::{With, Without};
use dyn_cnv_asset::asset::AssetContent;
use dyn_cnv_bundles::{
    components::marker::Root,
    events::{CoreInputEvent, ExecuteLuaScriptInputEvent},
    reference_id::ReferenceId,
};
use dyn_cnv_core::{resources::canvas::CanvasRes, CnvCorePlugin};
use dyn_cnv_dtif::DtifCanvas;
use dyn_cnv_lua::tables::args_table::LuaScriptArgsMap;
use dyn_cnv_svg_builder::{
    events::SvgBuilderOutputEvent, svg::svg_bundle::SvgBundleVariant, CnvSvgBuilderPlugin,
};
use resvg::usvg::Options;
use serde::Deserialize;
use std::{collections::HashMap, sync::mpsc::channel};
use usvg::WriteOptions;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(handler))
}

#[derive(Deserialize, utoipa::IntoParams)]
#[serde(rename_all = "camelCase")]
struct QueryParams {
    format: Option<FileFormat>,
    #[serde(rename(deserialize = "s"))]
    script_args: Option<HashMap<String, LuaScriptArgsMap>>,
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
    path = "/v1/cnv/render",
    operation_id = "post_v1_cnv_render_handler",
    params(
        QueryParams,
    ),
    request_body =DtifCanvas,
    responses(
        (status = 200, description = "Generation success", body = String),
        (status = BAD_REQUEST, description = "Bad Request", body = AppError)
    ),
)]
#[axum::debug_handler]
async fn handler(
    app_query: AppQuery<QueryParams>,
    app_body: AppJson<DtifCanvas>,
) -> Result<Response, AppError> {
    let QueryParams {
        format: maybe_format,
        script_args: maybe_script_args,
    } = app_query.get();
    let mut dtif = app_body.get();

    if let Some(script_args) = maybe_script_args {
        for (id, args) in script_args {
            dtif.events.push(CoreInputEvent::ExecuteLuaScript(
                ExecuteLuaScriptInputEvent {
                    id: ReferenceId::new(id),
                    args_map: args,
                },
            ))
        }
    }

    prepare_dtif_canvas(&mut dtif).await.map_err(|err| {
        AppError::new_with_options(
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::new("PREPARE_DTIF"),
            AppErrorOptions {
                description: Some(err.to_string()),
                ..Default::default()
            },
        )
    })?;
    let svg_string = build_svg_string(dtif)?;

    // Determine response format from query parameter
    match maybe_format.unwrap_or(FileFormat::Png) {
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

async fn prepare_dtif_canvas(dtif_canvas: &mut DtifCanvas) -> Result<(), reqwest::Error> {
    for asset in dtif_canvas.assets.iter_mut() {
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

fn build_svg_string(mut dtif: DtifCanvas) -> Result<String, AppError> {
    let mut app = App::new();

    let (svg_builder_output_event_sender, _) = channel::<SvgBuilderOutputEvent>();

    // Register plugins
    app.add_plugins((
        CnvCorePlugin {
            version: dtif.version,
            size: dtif.size,
            viewport: dtif.viewport,
        },
        CnvSvgBuilderPlugin {
            output_event_sender: svg_builder_output_event_sender,
        },
    ));

    dtif.send_into_world(&mut app.world);
    app.update();

    let mut result = String::new();
    let cnv_res = app.world.get_resource::<CanvasRes>().ok_or(AppError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        ErrorCode::new("CANVAS_RES_NOT_FOUND"),
    ))?;

    // Open SVG tag
    result.push_str(&format!(
        "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
        cnv_res.size.width(),
        cnv_res.size.height()
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
