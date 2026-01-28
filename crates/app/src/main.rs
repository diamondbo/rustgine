//! Application entry point.
//!
//! Initializes configuration, tracing, and runs the main event loop.

use app::resources::{run, AppState};
use core::{init_tracing, Config};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    init_tracing(&config.environment);
    info!(
        environment = %config.environment,
        service = "rustgine",
        "starting"
    );

    let state = AppState::initialize(&config).await?;

    run(state).await?;

    info!(
        environment = %config.environment,
        service = "rustgine",
        "shutting down"
    );
    Ok(())
}
