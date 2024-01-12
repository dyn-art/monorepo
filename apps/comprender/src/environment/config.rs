use dotenv_config::EnvConfig;
use dotenvy::dotenv;

#[derive(Debug, EnvConfig)]
pub struct Config {
    #[env_config(name = "APP_PORT", default = 3000)]
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Self, &'static str> {
        // Load .env file
        dotenv().ok();

        // Initialize configuration
        return Config::init();
    }
}
