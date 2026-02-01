//! Structured logging and tracing infrastructure.
//!
//! Provides environment-aware logging initialization using the [`tracing`]
//! ecosystem for structured, high-performance observability.

use tracing_subscriber::{fmt, EnvFilter};

/// Initializes the global tracing subscriber with environment-based filtering.
///
/// Sets up structured logging with automatic log level selection based on
/// the runtime environment. The `RUST_LOG` environment variable can override
/// the default log level.
///
/// # Log Levels by Environment
///
/// | Environment | Default Level |
/// |-------------|---------------|
/// | dev         | debug         |
/// | staging     | info          |
/// | prod        | warn          |
/// | (other)     | info          |
///
/// # Panics
///
/// Panics if called more than once, as the global subscriber can only be
/// set once per process.
///
/// # Example
///
/// ```
/// use core::init_tracing;
///
/// // Initialize with development defaults
/// // init_tracing("dev");
/// ```
pub fn init_tracing(env: &str) {
    let default_level = match env.to_ascii_lowercase().as_str() {
        "dev" | "development" => "debug",
        "prod" | "production" => "warn",
        _ => "info",
    };

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .init();
}
