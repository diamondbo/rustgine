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
    pub rustgine_systems: Vec<Box<dyn core::RustgineSystem + Send + Sync>>,
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
            rustgine_systems: Vec::new(),
        }))
    }

    pub fn register_system<S: core::RustgineSystem + Send + Sync + 'static>(&mut self, system: S) {
        self.rustgine_systems.push(Box::new(system));
    }
}
