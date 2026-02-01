//! Platform abstraction layer for the Rustgine game engine.
//!
//! This crate provides OS and windowing system abstractions, enabling
//! cross-platform window management, input handling, and system integration.
//!
//! # Overview
//!
//! The platform crate handles:
//! - Window creation and lifecycle management
//! - Input event collection (keyboard, mouse, gamepad)
//! - OS-level integration (clipboard, file dialogs, etc.)
//!
//! # Example
//!
//! ```ignore
//! use platform::RustginePlatform;
//! use rustgine_core::RustgineSystem;
//!
//! let mut platform = RustginePlatform::default();
//! platform.startup()?;
//! ```

#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod platform;

pub use platform::RustginePlatform;
