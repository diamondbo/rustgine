//! Application entry point.
//!
//! Initializes configuration, tracing, and runs the main event loop.

use core::{Config, init_tracing};
use tracing::info;
use app::resources::{AppState, run};

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
