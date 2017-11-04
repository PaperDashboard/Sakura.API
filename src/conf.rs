use config::{ConfigError, Config, File};
use std::env;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub db: String,
}

#[derive(Debug, Deserialize)]
pub struct Security {
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub security: Security,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("conf/default.toml"))?;
        
        let env = env::var("ENV").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        s.try_into()
    }
}