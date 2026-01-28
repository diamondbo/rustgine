use crate::resources::Shutdown;
use core::Config;
use std::sync::Arc;

/// Global application state shared across tasks.
///
/// Holds configuration and shutdown signaling for the app.
pub struct AppState {
    /// Shared application configuration.
    pub config: Arc<Config>,
    /// Graceful shutdown signal broadcaster.
    pub shutdown: Shutdown,
}

impl AppState {
    /// Initialize the application state.
    ///
    /// # Arguments
    /// * `config` - Shared application configuration (Arc).
    ///
    /// # Returns
    /// An `Arc<AppState>` ready for use by the application.
    pub async fn initialize(config: &Config) -> anyhow::Result<Arc<Self>> {
        Ok(Arc::new(Self {
            config: Arc::new(config.clone()),
            shutdown: Shutdown::new(),
        }))
    }
}
