use dotenv_config::EnvConfig;
use dotenvy::dotenv;

#[derive(Debug, Clone, EnvConfig)]
pub struct AppConfig {
    #[env_config(name = "APP_PORT", default = 3005)]
    pub port: u16,
    #[env_config(name = "CARGO_PKG_VERSION")]
    pub pkg_version: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, &'static str> {
        // Load .env file into environment
        match dotenv() {
            Ok(path) => println!(
                "⬜️ Loaded .env file from: {}",
                path.into_os_string()
                    .into_string()
                    .unwrap_or("unkown".to_string())
            ),
            Err(_) => println!("🟨 No .env file found!"),
        }

        // Load required environment variables into a struct
        return AppConfig::init();
    }
}
