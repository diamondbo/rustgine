//! Platform abstraction implementation.
//!
//! Provides the [`RustginePlatform`] system for managing window creation,
//! input handling, and OS-level interactions.

use rustgine_core::RustgineSystem;

/// Platform abstraction layer for the Rustgine engine.
///
/// Responsible for:
/// - Window creation and management
/// - Input event polling and dispatch
/// - OS-specific functionality abstraction
///
/// # Thread Safety
///
/// This type is designed to be used from a single thread (typically the main thread)
/// as required by most windowing systems.
///
/// # Example
///
/// ```ignore
/// use platform::RustginePlatform;
/// use rustgine_core::RustgineSystem;
///
/// let mut platform = RustginePlatform;
/// platform.startup()?;
/// // ... run game loop ...
/// platform.shutdown()?;
/// ```
#[derive(Debug, Default)]
pub struct RustginePlatform;

impl RustgineSystem for RustginePlatform {
    /// Initializes the platform subsystem.
    ///
    /// # Errors
    ///
    /// Returns an error if platform initialization fails (e.g., window creation fails).
    #[inline]
    fn startup(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Shuts down the platform subsystem and releases resources.
    ///
    /// # Errors
    ///
    /// Returns an error if cleanup fails.
    #[inline]
    fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
