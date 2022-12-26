use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use log::info;
use log::LevelFilter;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_derive::Deserialize;
use std::env;

/// The default `Config` instance
static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("Unable to retrieve config"));

/// Application Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// The application's run mode (typically "development" or "production")
    pub run_mode: String,
    /// The port to bind to
    pub port: u16,
    /// Database config
    pub database: Database,
}

/// Database config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    /// Full database url
    pub url: String,
    /// Database debug logging
    pub debug: bool,
    /// Database pool config
    pub pool: DbPool,
}

/// Database pool config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbPool {
    /// Database pool min
    pub min: i16,
    /// Database pool max
    pub max: i16,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            run_mode: "production".to_string(),
            port: 50051,
            database: Database {
                url: "mysql://user:password@db:3306/ledger".to_string(),
                debug: false,
                pool: DbPool { min: 1, max: 2 },
            },
        }
    }
}

impl Config {
    /// Create a new Config by merging in various sources
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "production".to_string());
        info!("Current mode {}", run_mode);

        let config: Config = Figment::from(Serialized::defaults(Config::default()))
            // Load run mode overrides
            .merge(Toml::file(format!("config/{}.toml", run_mode)))
            // Load environment variables
            .merge(
                // Support the nested structure of the config manually
                Env::raw()
                    // Split the Database variables
                    .map(|key| key.as_str().replace("DB_POOL_", "DATABASE.POOL.").into())
                    .map(|key| key.as_str().replace("DB_", "DATABASE.").into()),
            )
            // Serialize and freeze
            .extract()?;

        Ok(config)
    }

    /// Return true if the `run_mode` is "development"
    pub fn is_dev(&self) -> bool {
        self.run_mode == "development"
    }
}

/// Get the default static `Config`
pub fn get_config() -> &'static Config {
    &CONFIG
}

pub fn get_log_level_filter(log_level: String) -> LevelFilter {
    match log_level.to_lowercase().as_str() {
        "debug" => LevelFilter::Debug,
        "error" => LevelFilter::Error,
        "info" => LevelFilter::Info,
        "off" => LevelFilter::Off,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Warn,
    }
}
