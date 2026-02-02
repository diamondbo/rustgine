//! Rustgine application entry point.
//!
//! Initializes the engine configuration, tracing infrastructure, and
//! runs the main event loop until shutdown.
//!
//! # Exit Codes
//!
//! - `0` - Clean shutdown
//! - `1` - Error during initialization or runtime

use app::resources::{run, AppState};
use platform::RustginePlatform;
use render::RustgineRender;
use rustgine_core::{init_tracing, Config};
use scheduler::RustgineScheduler;
use tracing::info;

/// Application entry point.
///
/// Performs the following initialization sequence:
///
/// 1. Load configuration from environment
/// 2. Initialize structured logging/tracing
/// 3. Create application state
/// 4. Run the main event loop
/// 5. Log shutdown and exit
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration first (before tracing, as it may affect log levels)
    let config = Config::load()?;

    // Initialize tracing with environment-appropriate defaults
    init_tracing(&config.log_level);

    info!(
        environment = %config.environment,
        log_level = %config.log_level,
        service = "rustgine",
        "engine starting"
    );

    // Initialize application state and run
    let state = AppState::initialize(&config)?;

    // Initialize subsystems in dependency order
    let platform = RustginePlatform;
    let render = RustgineRender;
    let scheduler = RustgineScheduler;

    state.register_system("platform", platform)?;
    state.register_system("render", render)?;
    state.register_system("scheduler", scheduler)?;

    // Run the main event loop
    run(state).await?;
    info!(
        environment = %config.environment,
        service = "rustgine",
        "engine shutdown complete"
    );

    Ok(())
}
