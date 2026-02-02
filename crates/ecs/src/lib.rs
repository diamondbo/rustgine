//! Entity Component System for the Rustgine game engine.
//!
//! This crate provides a data-oriented ECS architecture for efficient
//! game object management and system execution.
//!
//! # Overview
//!
//! The ECS crate handles:
//! - Entity lifecycle management (creation, destruction, queries)
//! - Component storage with cache-friendly memory layouts
//! - System execution with automatic parallelization
//!
//! # Example
//!
//! ```ignore
//! use ecs::RustgineEcs;
//! use rustgine_core::RustgineSystem;
//!
//! let mut ecs = RustgineEcs::default();
//! ecs.startup()?;
//! ```

#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod ecs;

pub use ecs::RustgineEcs;
