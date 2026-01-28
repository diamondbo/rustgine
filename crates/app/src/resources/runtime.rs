use crate::resources::AppState;
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
    // Subscribe to the shutdown signal for this task.
    let mut shutdown_rx = state.shutdown.subscribe();

    // Pin the shutdown future to satisfy tokio::select! requirements
    let mut shutdown_fut = Box::pin(shutdown_rx.recv());

    // Wait for either Ctrl+C or internal shutdown.
    tokio::select! {
        // Handle external OS signal (Ctrl+C)
        _ = tokio::signal::ctrl_c() => {
            info!("shutdown signal received");
            // Notify all tasks to shut down
            state.shutdown.trigger();
        }
        // Handle internal shutdown (e.g., from another task)
        _ = &mut shutdown_fut => {
            info!("internal shutdown triggered");
        }
    }

    // Allow background tasks to finish cleanly
    info!("waiting for tasks to shut down");

    Ok(())
}
