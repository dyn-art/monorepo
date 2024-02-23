use specta::{
    export::ts_with_cfg,
    ts::{BigIntExportBehavior, ExportConfig},
};
use specta_test::*;

#[derive(specta::Type, serde::Serialize, serde::Deserialize)]
pub struct InCli {
    hello: String,
}

fn main() {
    let output_path = "./bindings.ts";
    let export_config = ExportConfig::default().bigint(BigIntExportBehavior::Number);
    match ts_with_cfg(output_path, "".into(), &export_config) {
        Ok(_) => println!(
            "✅ Successfully generated TypeScript types at {}",
            output_path
        ),
        Err(error) => {
            eprintln!("🟥 Failed to generate TypeScript types: {}", error);
            std::process::exit(1);
        }
    }
}
