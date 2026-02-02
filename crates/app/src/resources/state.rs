//! Global application state management.
//!
//! Provides the central state container that holds configuration,
//! subsystem references, and shutdown coordination.

use crate::resources::Shutdown;
use rustgine_core::{Config, RustgineSystem};
use std::sync::{Arc, Mutex};

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
/// async tasks. Subsystem registration and access are synchronized
/// internally to allow registration without exclusive access.
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
    ///
    /// A mutex provides interior mutability so subsystems can be registered
    /// without requiring a mutable reference to `AppState`.
    // rustgine_systems: Vec<Box<dyn RustgineSystem + Send + Sync>>,
    pub rustgine_systems: Mutex<Vec<NamedSystem>>,
}

/// Named wrapper for engine subsystems.
///
/// Associates a human-readable name with each subsystem for logging
/// and management purposes.
#[derive(Debug)]
pub struct NamedSystem {
    pub name: String,
    pub enabled: bool,
    pub system: Box<dyn RustgineSystem + Send + Sync>,
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
            rustgine_systems: Mutex::new(Vec::new()),
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
    /// state.register_system("platform", RustginePlatform::default())?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the subsystem registry lock is poisoned.
    pub fn register_system<S>(&self, alias: &str, system: S) -> anyhow::Result<()>
    where
        S: RustgineSystem + Send + Sync + 'static,
    {
        let mut systems = self
            .rustgine_systems
            .lock()
            .map_err(|_| anyhow::anyhow!("rustgine systems lock poisoned"))?;

        systems.push(NamedSystem {
            name: alias.to_string(),
            enabled: true,
            system: Box::new(system),
        });

        Ok(())
    }

    /// Returns the number of registered subsystems.
    ///
    /// Returns `0` if the subsystem registry lock is poisoned.
    #[must_use]
    #[inline]
    pub fn system_count(&self) -> usize {
        self.rustgine_systems
            .lock()
            .map(|systems| systems.len())
            .unwrap_or(0)
    }
}
