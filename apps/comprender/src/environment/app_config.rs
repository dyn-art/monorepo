use dotenv_config::EnvConfig;
use dotenvy::dotenv;

#[derive(Debug, EnvConfig)]
pub struct AppConfig {
    #[env_config(name = "APP_PORT", default = 3005)]
    pub port: u16,
}

impl AppConfig {
    pub fn new() -> Result<Self, &'static str> {
        dotenv().ok();
        return AppConfig::init();
    }
}
