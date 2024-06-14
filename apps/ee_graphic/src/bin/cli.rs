#[cfg(feature = "cli")]
mod cli {
    use clap::Parser;
    use dyn_graphic::api_doc::ApiDocs;
    use std::fs;

    #[derive(Parser, Debug)]
    #[clap(name = "Canvas Render CLI")]
    struct Cli {
        #[clap(subcommand)]
        pub cmd: SubCommand,
    }

    #[derive(Parser, Debug)]
    enum SubCommand {
        GenerateOpenAPI(GenerateOpenAPI),
    }

    #[derive(Parser, Debug)]
    struct GenerateOpenAPI {
        /// Path to save the generated OpenAPI file, defaults to "./openapi.yaml"
        #[clap(long, default_value = "./openapi.yaml")]
        output_path: String,
    }

    fn generate_openapi(output_path: &str) {
        // Generate OpenAPI JSON
        let openapi_json = ApiDocs::generate();

        // Write to file and handle potential errors
        match fs::write(output_path, &openapi_json) {
            Ok(_) => println!(
                "✅ Successfully generated OpenAPI file at {:?}",
                output_path
            ),
            Err(error) => eprintln!("🟥 Failed to write OpenAPI JSON to file: {}", error),
        }
    }

    pub fn run() {
        let args = Cli::parse();
        match args.cmd {
            SubCommand::GenerateOpenAPI(sub_args) => {
                generate_openapi(&sub_args.output_path);
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
