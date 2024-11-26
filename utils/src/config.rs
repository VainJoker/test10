use std::{
    fs,
    path::Path,
    sync::OnceLock,
};

use realme::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

pub static CFG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub app:       AppConfig,
    pub env:       EnvConfig,
    pub log:       LogConfig,
    pub database:  DatabaseConfig,
    pub rabbitmq:  RabbitMQConfig,
    pub dragonfly: DragonflyConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub api_host: String,
    pub api_port: usize,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum EnvConfig {
    Development,
    Production,
}

impl<'de> Deserialize<'de> for EnvConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "development" | "dev" => Ok(Self::Development),
            "production" | "prod" => Ok(Self::Production),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid environment: {s}"
            ))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    pub dir:      String,
    pub database: LogInnerConfig,
    pub error:    LogInnerConfig,
    pub other:    LogInnerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogInnerConfig {
    pub dir:    Option<String>,
    pub prefix: String,
    pub level:  String,
    pub target: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RabbitMQConfig {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DragonflyConfig {
    pub url: String,
}

/// Initializes the application's configuration from the provided file.
/// Expected to be run on startup of the application.
pub fn init(cfg_file: impl AsRef<Path>) {
    // Attempt to extract the canonical, absolute path of the configuration
    // file. Panic if this operation fails, as the configuration is critical
    // for execution.
    let path = fs::canonicalize(&cfg_file).unwrap_or_else(|e| {
        panic!(
            "💥 Failed to load configuration file: {e} - {}",
            cfg_file.as_ref().display()
        );
    });

    // Attempt to build the configuration from the file.
    // Panic if any errors occur during loading or validation.
    let realme = Realme::builder()
        .load(Adaptor::new(FileSource::<TomlParser>::new(path)))
        .build()
        .unwrap_or_else(|e| {
            panic!("💥 Failed to build configuration: {e}");
        });

    let config: Config = realme.try_deserialize().unwrap_or_else(|e| {
        panic!("💥 Failed to deserialize configuration: {e}");
    });
    // Attempt to lock the configuration for the first time.
    // Ignore the result because we'd panic if locking fails.
    let _ = CFG.set(config);
    tracing::info!("🚀 Configuration loading is successful!");
}

/// Accesses the application's configuration, once initialized.
/// Panics if called before `init`.
pub fn config() -> &'static Config {
    CFG.get().unwrap_or_else(|| {
        panic!("💥 Configuration accessed before initialization");
    })
}
