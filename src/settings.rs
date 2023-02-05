use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mail {
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HomeAssistant {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub mail: Mail,
    pub home_assistant: HomeAssistant,
}

const CONFIG_FILE: &str = "settings.toml";

impl Settings {
    pub fn new(settings_file: Option<&str>) -> Result<Self, ConfigError> {
        let settings_file = settings_file.unwrap_or(CONFIG_FILE);
        let cfg = Config::builder()
            .set_default("server.port", "3000")?
            .set_default("mail.host", "localhost")?
            .add_source(File::new(settings_file, FileFormat::Toml))
            .add_source(Environment::with_prefix("M").separator("_"))
            .build()?;

        cfg.try_deserialize()
    }
}
