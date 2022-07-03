use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
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
    pub mail: Mail,
    pub home_assistant: HomeAssistant,
}

const CONFIG_FILE: &str = "settings.toml";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let cfg = Config::builder()
            .set_default("server.port", "3000")?
            .set_default("mail.host", "localhost")?
            .add_source(File::new(CONFIG_FILE, FileFormat::Toml))
            .build()?;

        cfg.try_deserialize()
    }
}
