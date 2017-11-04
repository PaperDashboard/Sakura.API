use config::{ConfigError, Config, File};
use std::env;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
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

        s.set("database.url", "mongodb://localhost/sakura")?;
        s.set("security.key", "ChangeMe")?;

        s.try_into()
    }
}