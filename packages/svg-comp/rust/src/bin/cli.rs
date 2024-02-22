#[cfg(feature = "cli")]
mod cli {
    use clap::Parser;
    use specta::ts::{BigIntExportBehavior, ExportConfig};

    // Import all types and modules from `dyn_svg_comp_api` to generate types from
    use dyn_svg_comp_api::*;

    #[derive(Parser, Debug)]
    #[clap(name = "SVG Composition CLI")]
    struct Cli {
        #[clap(subcommand)]
        pub cmd: SubCommand,
    }

    #[derive(Parser, Debug)]
    enum SubCommand {
        GenerateTsTypes(GenerateTsTypes),
    }

    #[derive(Parser, Debug)]
    struct GenerateTsTypes {
        /// Path to save the generated Typescript types file, default to "./bindings.ts"
        #[clap(long, default_value = "./bindings.ts")]
        output_path: String,
    }

    fn generate_ts_types(output_path: &str) {
        println!("🚀 Generating TypeScript types at {}", output_path);
        let export_config = ExportConfig::default().bigint(BigIntExportBehavior::Number);
        match specta::export::ts_with_cfg(output_path, "".into(), &export_config) {
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

    pub fn run() {
        let args = Cli::parse();
        match args.cmd {
            SubCommand::GenerateTsTypes(sub_args) => {
                generate_ts_types(&sub_args.output_path);
            }
        }
    }
}

#[cfg(feature = "cli")]
fn main() {
    cli::run();
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("🟥 This program requires the 'cli' feature. Please run with '--feature cli'.");
}
