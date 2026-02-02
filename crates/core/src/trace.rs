//! Structured logging and tracing infrastructure.
//!
//! Provides logging initialization using the [`tracing`] ecosystem for
//! structured, high-performance observability.
use std::sync::Once;
use tracing_subscriber::{fmt, EnvFilter};

static INIT_TRACING: Once = Once::new();
/// Initializes the global tracing subscriber with the given default log level.
///
/// Sets up structured logging with the specified log level as the default.
/// The `RUST_LOG` environment variable can override this default, allowing
/// runtime customization without recompilation.
///
/// This function is idempotent: subsequent calls after the first are no-ops.
///
/// # Arguments
///
/// * `log_level` - Default log level filter (e.g., `"info"`, `"debug"`, `"warn"`)
///
/// # Example
///
/// ```ignore
/// use core::init_tracing;
///
/// // Initialize with info level; RUST_LOG=debug overrides at runtime
/// init_tracing("info");
/// ```
pub fn init_tracing(log_level: &str) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    INIT_TRACING.call_once(|| {
        fmt()
            .with_env_filter(filter)
            .with_target(true)
            .with_thread_ids(false)
            .with_thread_names(false)
            .init();
    });
}
