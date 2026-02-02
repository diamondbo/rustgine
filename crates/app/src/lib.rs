//! Application framework for the Rustgine game engine.
//!
//! This crate provides the high-level application structure including
//! state management, runtime execution, and graceful shutdown handling.
//!
//! # Overview
//!
//! The app crate orchestrates all engine subsystems and provides:
//!
//! - [`AppState`](resources::AppState) - Global application state shared across tasks
//! - [`Shutdown`](resources::Shutdown) - Graceful shutdown signal broadcasting
//! - [`run`](resources::run) - Main application event loop
//!
//! # Architecture
//!
//! ```text
//! ┌────────────────────────────────────────────────┐
//! │                    App                         │
//! │  ┌───────────┐  ┌──────────┐  ┌────────────┐   │
//! │  │ AppState  │  │ Shutdown │  │  Runtime   │   │
//! │  └───────────┘  └──────────┘  └────────────┘   │
//! └────────────────────────────────────────────────┘
//!           │              │              │
//!     ┌─────┴─────┐  ┌─────┴─────┐  ┌─────┴────┐
//!     │  Config   │  │  Signal   │  │Subsystems│
//!     └───────────┘  └───────────┘  └──────────┘
//! ```
//!
//! # Example
//!
//! ```ignore
//! use app::resources::{AppState, run};
//! use rustgine_core::Config;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = Config::load()?;
//!     let state = AppState::initialize(&config)?;
//!     run(state).await?;
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod resources;
