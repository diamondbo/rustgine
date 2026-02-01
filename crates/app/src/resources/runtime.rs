use crate::resources::AppState;
use rustgine_core::RustgineSystem;
use platform::RustginePlatform;
use render::RustgineRender;
use scheduler::RustgineScheduler;
use std::sync::Arc;
use tracing::info;

/// Runs the main application event loop, handling shutdown signals.
///
/// Listens for either an external OS signal (Ctrl+C) or an internal shutdown trigger,
/// and coordinates graceful shutdown of the application.
///
/// # Arguments
/// * `state` - Shared application state (Arc<AppState>).
///
/// # Returns
/// * `anyhow::Result<()>` - Ok on clean shutdown, Err on failure.
pub async fn run(state: Arc<AppState>) -> anyhow::Result<()> {
    // Create and start subsystems
    let mut platform = RustginePlatform;
    let mut render = RustgineRender;
    let mut scheduler = RustgineScheduler;

    platform.startup()?;
    render.startup()?;
    scheduler.startup()?;

    // Subscribe to the shutdown signal for this task.
    let mut shutdown_rx = state.shutdown.subscribe();
    let mut shutdown_fut = Box::pin(shutdown_rx.recv());

    // Wait for either Ctrl+C or internal shutdown.
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("shutdown signal received");
            state.shutdown.trigger();
        }
        _ = &mut shutdown_fut => {
            info!("internal shutdown triggered");
        }
    }

    // Allow background tasks to finish cleanly
    info!("waiting for tasks to shut down");

    // Shutdown subsystems (reverse order)
    scheduler.shutdown()?;
    render.shutdown()?;
    platform.shutdown()?;

    Ok(())
}
