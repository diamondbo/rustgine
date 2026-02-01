//! Task scheduling subsystem for the Rustgine game engine.
//!
//! This crate provides parallel task execution and job scheduling
//! for efficient multi-threaded workloads.
//!
//! # Overview
//!
//! The scheduler crate handles:
//! - Parallel task execution across CPU cores
//! - Job dependency management
//! - Work stealing for optimal load distribution
//! - Frame-synchronized task scheduling
//!
//! # Example
//!
//! ```ignore
//! use scheduler::RustgineScheduler;
//! use rustgine_core::RustgineSystem;
//!
//! let mut scheduler = RustgineScheduler::default();
//! scheduler.startup()?;
//! ```

#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod scheduler;

pub use scheduler::RustgineScheduler;
