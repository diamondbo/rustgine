//! Rendering subsystem for the Rustgine game engine.
//!
//! This crate provides GPU-accelerated graphics rendering capabilities
//! using modern graphics APIs.
//!
//! # Overview
//!
//! The render crate handles:
//! - Graphics device initialization and management
//! - Render pipeline creation and configuration
//! - Draw call submission and frame presentation
//! - GPU resource management (buffers, textures, shaders)
//!
//! # Example
//!
//! ```ignore
//! use render::RustgineRender;
//! use rustgine_core::RustgineSystem;
//!
//! let mut renderer = RustgineRender::default();
//! renderer.startup()?;
//! ```

#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod render;

pub use render::RustgineRender;
