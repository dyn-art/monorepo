#![cfg(feature = "cli")]

use clap::Parser;
use specta::export;
use specta::ts::{BigIntExportBehavior, ExportConfig};
use std::process;

// Import all types and modules from `dyn_composition` to make them accessible to specta here
use dyn_composition_api::*;

// Root CLI argument structure
#[derive(Parser, Debug)]
#[clap(name = "cli")]
struct Cli {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

// Enum for available subcommands
#[derive(Parser, Debug)]
enum SubCommand {
    GenerateTsTypes(GenerateTsTypes),
}

// Subcommand for generating TypeScript types
#[derive(Parser, Debug)]
struct GenerateTsTypes {
    #[clap(long)]
    export_path: String,
}

fn generate_ts_types(export_path: &str) {
    println!("Generating TypeScript types at {}", export_path);
    let export_config = ExportConfig::default().bigint(BigIntExportBehavior::Number);
    match export::ts_with_cfg(export_path, &export_config) {
        Ok(_) => println!("Successfully generated TypeScript types at {}", export_path),
        Err(e) => {
            eprintln!("Failed to generate TypeScript types: {:?}", e);
            process::exit(1);
        }
    }
}

fn main() {
    let args = Cli::parse();
    match args.cmd {
        SubCommand::GenerateTsTypes(sub_args) => {
            generate_ts_types(&sub_args.export_path);
        }
    }
}
