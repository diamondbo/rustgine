//! Core infrastructure for the Rustgine game engine.
//!
//! This crate provides foundational types and utilities shared across all
//! Rustgine subsystems, including configuration management, logging/tracing
//! infrastructure, and the system lifecycle trait.
//!
//! # Overview
//!
//! - [`Config`] - Application configuration loaded from environment variables
//! - [`RustgineSystem`] - Trait defining the lifecycle of engine subsystems
//! - [`init_tracing`] - Initializes structured logging with environment-based filtering
//!
//! # Example
//!
//! ```
//! use core::{Config, init_tracing};
//!
//! let config = Config::load().expect("Failed to load config");
//! init_tracing(&config.environment);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod config;
#[cfg(test)]
mod config_test;
pub mod system;
pub mod trace;
#[cfg(test)]
mod trace_test;

pub use config::Config;
pub use system::RustgineSystem;
pub use trace::init_tracing;
