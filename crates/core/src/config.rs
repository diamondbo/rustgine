/// Application configuration settings.
///
/// Holds environment and logging configuration for the app.
#[derive(Debug, Clone)]
pub struct Config {
    /// The environment the app is running in (e.g., "development", "production").
    pub environment: String,
    /// The log level for application logging (e.g., "info", "debug").
    pub log_level: String,
}

impl Config {
    /// Load configuration from environment or file.
    ///
    /// In production, this should parse environment variables or config files.
    /// For now, returns a default development config.
    pub fn load() -> anyhow::Result<Config> {
        use std::env;

        // Read environment variable, default to "development"
        let environment = env::var("NEUVIA_ENV").unwrap_or_else(|_| "development".to_string());

        // Set log level based on environment
        let log_level = if environment.eq_ignore_ascii_case("production") {
            "info".to_string()
        } else {
            "debug".to_string()
        };

        Ok(Config {
            environment,
            log_level,
        })
    }
}
