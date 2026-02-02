//! Application resources and runtime management.
//!
//! This module contains the core building blocks for the application:
//!
//! - [`AppState`] - Global state container for configuration and subsystems
//! - [`Shutdown`] - Graceful shutdown signal broadcaster
//! - [`run`] - Main event loop execution

mod runtime;
mod shutdown;
#[cfg(test)]
mod shutdown_test;
mod state;

pub use runtime::run;
pub use shutdown::{Shutdown, ShutdownRx};
pub use state::AppState;
