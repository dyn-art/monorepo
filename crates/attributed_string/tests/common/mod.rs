use dyn_attributed_string::{
    layout::layouter::{Layouter, LayouterConfig},
    outline::tiny_skia_path_builder::TinySkiaPathBuilder,
    AttributedString,
};
use dyn_fonts_book::FontsBook;
use dyn_utils::properties::size::Size;
use std::path::PathBuf;
use tiny_skia::{Color, FillRule, Paint, Pixmap, Transform};

pub struct TestPaths {
    pub repo_dir: String,
    pub fonts_dir_path: PathBuf,
    pub images_dir_path: PathBuf,
}

impl TestPaths {
    pub fn new() -> Self {
        let repo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(
            // Fallback because in Debug mode the "CARGO_MANIFEST_DIR" env is not set
            std::env::var("ENV_ROOT_DIR")
                .map(|dir| format!("{}/crates/attributed_string", dir))
                .unwrap(),
        );
        let assets_dir_path = PathBuf::from(&repo_dir).join("tests").join("assets");
        let fonts_dir_path = assets_dir_path.join("fonts");
        let images_dir_path = assets_dir_path.join("images");
        Self {
            repo_dir,
            fonts_dir_path,
            images_dir_path,
        }
    }
}

pub fn assert_attributed_string_rendered(
    name: &'static str,
    attributed_string: &mut AttributedString,
    config: LayouterConfig,
) {
    init_env_logger();

    let test_paths = TestPaths::new();
    let image_file_path = test_paths.images_dir_path.join(format!("{}.png", name));
    let should_generate_images = std::env::var("GENERATE_IMAGES")
        .map(|v| {
            let val = v.trim().to_ascii_lowercase();
            ["true", "1"].iter().any(|&v| v == val)
        })
        .unwrap_or(false);

    let mut fonts_book = FontsBook::new();
    // fonts_book.load_system_fonts();
    fonts_book
        .get_db_mut()
        .load_fonts_dir(test_paths.fonts_dir_path);

    attributed_string.tokenize_text(&mut fonts_book);
    let mut layouter = Layouter::new(config);
    layouter.layout(attributed_string.get_spans_mut());
    let container_size = layouter.get_container_size().unwrap();

    let path =
        TinySkiaPathBuilder::outline(attributed_string.get_spans(), &mut fonts_book).unwrap();

    let mut paint = Paint::default();
    paint.set_color(Color::BLACK);
    paint.anti_alias = true;

    let mut pixmap = Pixmap::new(
        container_size.width() as u32,
        container_size.height() as u32,
    )
    .unwrap();
    pixmap.fill(Color::WHITE);
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::default(),
        Transform::identity(),
        None,
    );

    if should_generate_images {
        pixmap.save_png(image_file_path).unwrap();
    } else {
        let reference_image_data = std::fs::read(image_file_path).unwrap();
        let image_data = pixmap.encode_png().unwrap();
        assert_eq!(
            reference_image_data, image_data,
            "Rendered assertion of '{name}' failed!"
        )
    }
}

pub fn init_env_logger() {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .is_test(true)
        .try_init();
}
