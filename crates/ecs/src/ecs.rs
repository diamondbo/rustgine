//! Entity Component System implementation.
//!
//! Provides the [`RustgineEcs`] system for managing entities, components,
//! and system execution.

use rustgine_core::RustgineSystem;

/// Entity Component System subsystem for the Rustgine engine.
///
/// Manages:
/// - Entity creation and destruction
/// - Component storage and queries
/// - System scheduling and execution
///
/// # Example
///
/// ```ignore
/// use ecs::RustgineEcs;
/// use rustgine_core::RustgineSystem;
///
/// let mut ecs = RustgineEcs::default();
/// ecs.startup()?;
/// ```
#[derive(Debug, Default)]
pub struct RustgineEcs;

impl RustgineSystem for RustgineEcs {
    /// Initializes the ECS subsystem.
    ///
    /// # Errors
    ///
    /// Returns an error if ECS initialization fails.
    #[inline]
    fn startup(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Shuts down the ECS subsystem and releases resources.
    ///
    /// # Errors
    ///
    /// Returns an error if cleanup fails.
    #[inline]
    fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
