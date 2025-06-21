use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LlmConfig {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub passthrough_model_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub exposed_model_name: String,
    pub host: String,
    pub port: u16,
    pub llm_config: LlmConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config").required(false))
            .add_source(Environment::with_prefix("FXPIPE").separator("_"))
            .build()?;

        s.try_deserialize()
    }
}
