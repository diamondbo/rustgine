<!-- markdownlint-disable MD024 -->
# Changelog

All notable changes to Identra will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-01-25

### Added

- Versioning and changelog checks in CI pipeline
- Code linting and formatting checks in CI pipeline
- Changelog.md file with initial version history
- Versioning scripts for automating version bumps

## [0.1.0] - 2025-01-25

### Added

- Modular workspace structure with the following crates:
  - `core`: Shared abstractions and engine glue
  - `ecs`: Archetype-based Entity Component System
  - `scheduler`: Parallel system scheduler
  - `render`: WebGPU-based renderer
  - `platform`: Platform abstraction (windowing, input, timing)
  - `math`: Math primitives and utilities
  - `app`: Application layer and main loop
- Initial crate-level README files and library roots for all engine subsystems
- `docs/architecture.md`: High-level architecture and design principles
- MIT License file

### Changed

- Refactored project to a multi-crate Cargo workspace
- Updated main README with project overview, badges, and workspace layout
- CI pipeline refactored and renamed to `ci.yml`

### Removed

- Old monolithic `src/main.rs` (now replaced by modular crates)
