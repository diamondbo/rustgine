//! Application runtime and event loop management.
//!
//! Provides the main execution loop that coordinates all engine subsystems
//! and handles graceful shutdown on OS signals.

use crate::resources::AppState;
use platform::RustginePlatform;
use render::RustgineRender;
use rustgine_core::RustgineSystem;
use scheduler::RustgineScheduler;
use std::sync::Arc;
use tracing::{info, warn};

/// Runs the main application event loop.
///
/// This function orchestrates the engine lifecycle:
///
/// 1. **Startup**: Initializes all subsystems in dependency order
/// 2. **Run**: Waits for shutdown signal (Ctrl+C or internal trigger)
/// 3. **Shutdown**: Cleanly terminates subsystems in reverse order
///
/// # Arguments
///
/// * `state` - Shared application state containing configuration and shutdown coordinator
///
/// # Returns
///
/// Returns `Ok(())` on clean shutdown, or an error if any subsystem fails.
///
/// # Shutdown Triggers
///
/// The function will initiate shutdown when:
/// - `Ctrl+C` (SIGINT) is received from the OS
/// - The internal shutdown signal is triggered via [`Shutdown::trigger`](crate::resources::Shutdown::trigger)
///
/// # Example
///
/// ```ignore
/// use app::resources::{run, AppState};
/// use rustgine_core::Config;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let config = Config::load()?;
///     let state = AppState::initialize(&config).await?;
///     run(state).await
/// }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - Any subsystem fails during startup
/// - Any subsystem fails during shutdown
pub async fn run(state: Arc<AppState>) -> anyhow::Result<()> {
    // Initialize subsystems in dependency order
    let mut platform = RustginePlatform::default();
    let mut render = RustgineRender::default();
    let mut scheduler = RustgineScheduler::default();

    info!("starting platform subsystem");
    platform.startup()?;

    info!("starting render subsystem");
    render.startup()?;

    info!("starting scheduler subsystem");
    scheduler.startup()?;

    info!("all subsystems initialized, entering main loop");

    // Subscribe to shutdown signal for coordinated termination
    let mut shutdown_rx = state.shutdown.subscribe();
    let mut shutdown_fut = Box::pin(shutdown_rx.recv());

    // Wait for shutdown trigger (OS signal or internal)
    tokio::select! {
        result = tokio::signal::ctrl_c() => {
            match result {
                Ok(()) => info!("received Ctrl+C, initiating shutdown"),
                Err(e) => warn!(error = %e, "failed to listen for Ctrl+C signal"),
            }
            state.shutdown.trigger();
        }
        () = &mut shutdown_fut => {
            info!("internal shutdown signal received");
        }
    }

    info!("shutting down subsystems");

    // Shutdown in reverse dependency order
    if let Err(e) = scheduler.shutdown() {
        warn!(error = %e, "scheduler shutdown error");
    }

    if let Err(e) = render.shutdown() {
        warn!(error = %e, "render shutdown error");
    }

    if let Err(e) = platform.shutdown() {
        warn!(error = %e, "platform shutdown error");
    }

    info!("all subsystems shut down");
    Ok(())
}
