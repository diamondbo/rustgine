//! Application configuration management.
//!
//! Provides environment-aware configuration loading with sensible defaults
//! for development and production environments.

use std::env;

/// Environment variable name for specifying the runtime environment.
const ENV_VAR_NAME: &str = "RUSTGINE_ENV";

/// Default environment when none is specified.
const DEFAULT_ENVIRONMENT: &str = "development";

/// Application configuration settings.
///
/// Holds environment and logging configuration for the engine.
/// Configuration is typically loaded once at startup and shared
/// across all subsystems.
///
/// # Thread Safety
///
/// `Config` is [`Clone`] and can be safely shared across threads
/// when wrapped in an [`Arc`](std::sync::Arc).
///
/// # Example
///
/// ```
/// use core::Config;
///
/// let config = Config::load().expect("Failed to load config");
/// println!("Running in {} mode", config.environment);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// The runtime environment (e.g., "development", "staging", "production").
    ///
    /// Affects logging verbosity and other environment-specific behaviors.
    pub environment: String,

    /// The minimum log level for application logging.
    ///
    /// Common values: "trace", "debug", "info", "warn", "error".
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            environment: DEFAULT_ENVIRONMENT.to_owned(),
            log_level: "debug".to_owned(),
        }
    }
}

impl Config {
    /// Loads configuration from environment variables.
    ///
    /// Reads the `RUSTGINE_ENV` environment variable to determine the
    /// runtime environment. Log level is automatically set based on
    /// the environment:
    ///
    /// | Environment | Log Level |
    /// |-------------|----------|
    /// | development | debug    |
    /// | staging     | info     |
    /// | production  | info     |
    ///
    /// # Errors
    ///
    /// Currently this function is infallible, but returns `Result` to
    /// allow for future configuration sources that may fail (e.g., file I/O).
    ///
    /// # Example
    ///
    /// ```
    /// use core::Config;
    ///
    /// // With RUSTGINE_ENV unset, defaults to "development"
    /// let config = Config::load().unwrap();
    /// assert_eq!(config.environment, "development");
    /// ```
    pub fn load() -> anyhow::Result<Self> {
        let environment = env::var(ENV_VAR_NAME)
            .unwrap_or_else(|_| DEFAULT_ENVIRONMENT.to_owned());

        let log_level = Self::log_level_for_environment(&environment);

        Ok(Self {
            environment,
            log_level,
        })
    }

    /// Determines the appropriate log level for the given environment.
    #[must_use]
    fn log_level_for_environment(env: &str) -> String {
        match env.to_ascii_lowercase().as_str() {
            "production" | "prod" => "info",
            "staging" => "info",
            _ => "debug",
        }
        .to_owned()
    }

    /// Returns `true` if running in a development environment.
    #[must_use]
    #[inline]
    pub fn is_development(&self) -> bool {
        matches!(
            self.environment.to_ascii_lowercase().as_str(),
            "development" | "dev"
        )
    }

    /// Returns `true` if running in a production environment.
    #[must_use]
    #[inline]
    pub fn is_production(&self) -> bool {
        matches!(
            self.environment.to_ascii_lowercase().as_str(),
            "production" | "prod"
        )
    }
}
