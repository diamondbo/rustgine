//! Global application state management.
//!
//! Provides the central state container that holds configuration,
//! subsystem references, and shutdown coordination.

use crate::resources::Shutdown;
use rustgine_core::{Config, RustgineSystem};
use std::sync::Arc;

/// Global application state shared across all engine tasks.
///
/// `AppState` serves as the central hub for:
/// - Application configuration
/// - Graceful shutdown coordination
/// - Registered engine subsystems
///
/// # Thread Safety
///
/// `AppState` is designed to be wrapped in [`Arc`] and shared across
/// async tasks. Individual fields provide their own synchronization
/// where necessary.
///
/// # Lifecycle
///
/// 1. Create via [`initialize`](Self::initialize)
/// 2. Register subsystems via [`register_system`](Self::register_system)
/// 3. Pass to [`run`](crate::resources::run) for execution
/// 4. Shutdown is coordinated via the [`Shutdown`] broadcaster
///
/// # Example
///
/// ```ignore
/// use app::resources::AppState;
/// use rustgine_core::Config;
///
/// let config = Config::load()?;
/// let state = AppState::initialize(&config)?;
/// ```
#[derive(Debug)]
pub struct AppState {
    /// Shared application configuration.
    ///
    /// Wrapped in [`Arc`] to allow cheap cloning to subsystems.
    pub config: Arc<Config>,

    /// Graceful shutdown signal broadcaster.
    ///
    /// Used to coordinate shutdown across all engine tasks.
    pub shutdown: Shutdown,

    /// Registered engine subsystems.
    ///
    /// Systems are stored as trait objects to allow heterogeneous collections.
    /// They are started in registration order and shut down in reverse order.
    rustgine_systems: Vec<Box<dyn RustgineSystem + Send + Sync>>,
}

impl AppState {
    /// Initializes the application state with the given configuration.
    ///
    /// Creates a new `AppState` wrapped in [`Arc`] for sharing across tasks.
    ///
    /// # Arguments
    ///
    /// * `config` - The application configuration to use
    ///
    /// # Returns
    ///
    /// An `Arc<AppState>` ready for use, or an error if initialization fails.
    ///
    /// # Errors
    ///
    /// Currently infallible, but returns `Result` to allow for future
    /// initialization steps that may fail.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use app::resources::AppState;
    /// use rustgine_core::Config;
    ///
    /// let config = Config::load()?;
    /// let state = AppState::initialize(&config)?;
    /// ```
    pub fn initialize(config: &Config) -> anyhow::Result<Arc<Self>> {
        Ok(Arc::new(Self {
            config: Arc::new(config.clone()),
            shutdown: Shutdown::new(),
            rustgine_systems: Vec::new(),
        }))
    }

    /// Registers an engine subsystem for lifecycle management.
    ///
    /// Registered systems will be started during engine initialization
    /// and shut down during engine termination (in reverse order).
    ///
    /// # Type Parameters
    ///
    /// * `S` - A type implementing [`RustgineSystem`] + [`Send`] + [`Sync`]
    ///
    /// # Arguments
    ///
    /// * `system` - The subsystem instance to register
    ///
    /// # Example
    ///
    /// ```ignore
    /// use platform::RustginePlatform;
    ///
    /// // Note: requires mutable access, typically before Arc wrapping
    /// state.register_system(RustginePlatform::default());
    /// ```
    pub fn register_system<S>(&mut self, system: S)
    where
        S: RustgineSystem + Send + Sync + 'static,
    {
        self.rustgine_systems.push(Box::new(system));
    }

    /// Returns the number of registered subsystems.
    #[must_use]
    #[inline]
    pub fn system_count(&self) -> usize {
        self.rustgine_systems.len()
    }
}
