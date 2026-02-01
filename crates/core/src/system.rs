//! Engine system lifecycle management.
//!
//! Defines the [`RustgineSystem`] trait that all engine subsystems must implement
//! for proper initialization and cleanup.

use std::fmt::Debug;

/// Trait defining the lifecycle of an engine subsystem.
///
/// All major engine components (platform, renderer, scheduler, etc.) implement
/// this trait to ensure consistent initialization and shutdown behavior.
///
/// # Lifecycle
///
/// 1. **Startup**: Called once during engine initialization. Subsystems should
///    acquire resources, spawn threads, and prepare for operation.
///
/// 2. **Runtime**: The subsystem operates normally, processing frames or tasks.
///
/// 3. **Shutdown**: Called once during engine termination. Subsystems should
///    release resources, join threads, and clean up state.
///
/// # Ordering
///
/// Subsystems are typically started in dependency order and shut down in
/// reverse order to ensure proper resource cleanup.
///
/// # Example
///
/// ```
/// use core::RustgineSystem;
///
/// #[derive(Debug)]
/// struct AudioSystem {
///     initialized: bool,
/// }
///
/// impl RustgineSystem for AudioSystem {
///     fn startup(&mut self) -> anyhow::Result<()> {
///         // Initialize audio device, load banks, etc.
///         self.initialized = true;
///         Ok(())
///     }
///
///     fn shutdown(&mut self) -> anyhow::Result<()> {
///         // Stop playback, release audio device
///         self.initialized = false;
///         Ok(())
///     }
/// }
/// ```
pub trait RustgineSystem: Debug {
    /// Initializes the subsystem.
    ///
    /// Called once at engine startup. Implementations should acquire
    /// necessary resources and prepare for operation.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails. The engine will typically
    /// abort startup if any critical subsystem fails to initialize.
    fn startup(&mut self) -> anyhow::Result<()>;

    /// Shuts down the subsystem and releases resources.
    ///
    /// Called once at engine shutdown. Implementations should cleanly
    /// release all acquired resources.
    ///
    /// # Errors
    ///
    /// Returns an error if cleanup fails. Errors during shutdown are
    /// typically logged but may not prevent engine termination.
    fn shutdown(&mut self) -> anyhow::Result<()>;
}
