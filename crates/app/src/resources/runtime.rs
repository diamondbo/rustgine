//! Application runtime and event loop management.
//!
//! Provides the main execution loop that coordinates all engine subsystems
//! and handles graceful shutdown on OS signals.

use crate::resources::AppState;
use std::sync::Arc;
use tracing::{debug, warn};

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
///     let state = AppState::initialize(&config)?;
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
    {
        let mut systems = state
            .rustgine_systems
            .lock()
            .map_err(|_| anyhow::anyhow!("rustgine systems lock poisoned"))?;

        for system in systems.iter_mut() {
            if !system.enabled {
                debug!(system = %system.name, "subsystem disabled, skipping startup");
                continue;
            }
            debug!(system = %system.name, "starting subsystem");
            if let Err(e) = system.system.startup() {
                warn!(system = %system.name, error = %e, "failed to start subsystem");
                return Err(e);
            }
            debug!(system = %system.name, "subsystem started");
        }
    }
    debug!(systems = ?state.system_count(), "all subsystems initialized, entering main loop");

    // Subscribe to shutdown signal for coordinated termination
    let mut shutdown_rx = state.shutdown.subscribe();
    let mut shutdown_fut = Box::pin(shutdown_rx.recv());

    // Wait for shutdown trigger (OS signal or internal)
    tokio::select! {
        result = tokio::signal::ctrl_c() => {
            match result {
                Ok(()) => debug!("received Ctrl+C, initiating shutdown"),
                Err(e) => warn!(error = %e, "failed to listen for Ctrl+C signal"),
            }
            state.shutdown.trigger();
        }
        () = &mut shutdown_fut => {
            // Internal shutdown already triggered elsewhere; no need to re-trigger here.
            debug!("internal shutdown signal received");
        }
    }

    debug!("shutting down subsystems");

    // Shutdown in reverse dependency order
    let mut systems = state
        .rustgine_systems
        .lock()
        .map_err(|_| anyhow::anyhow!("rustgine systems lock poisoned"))?;

    for system in systems.iter_mut().rev() {
        if !system.enabled {
            debug!(system = %system.name, "subsystem disabled, skipping shutdown");
            continue;
        }
        debug!(system = %system.name, "shutting down subsystem");
        if let Err(e) = system.system.shutdown() {
            warn!(system = %system.name, error = %e, "failed to shut down subsystem");
            return Err(e);
        }
        debug!(system = %system.name, "subsystem shut down");
    }

    debug!("all subsystems shut down");
    Ok(())
}
