<!-- markdownlint-disable MD033 -->

<h1 align="center">Rustgine
<br>
<!-- Version Badge - do not edit manually -->
<a href="https://crates.io/crates/rustgine">
    <img src="https://img.shields.io/github/v/tag/diamondbo/rustgine?label=version" alt="Version">
</a>
</h1>

<p align="center">
  <i>A modern, data-oriented game engine written in Rust.</i><br><br>

<!-- CI Badges -->

<a href="https://github.com/diamondbo/rustgine/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue" alt="License: MIT">
</a>

<!-- License Badge -->

<a href="https://github.com/diamondbo/rustgine/actions/workflows/ci.yml">
    <img src="https://github.com/diamondbo/rustgine/actions/workflows/ci.yml/badge.svg" alt="CI">
</a>

</p>

**Rustgine** focuses on performance, safety, and scalability, using Rust’s type system and concurrency to enable high-performance simulation and rendering.

## Features

- Data-oriented, archetype-based ECS
- Parallel system scheduler
- Modular, multi-crate workspace
- WebGPU (wgpu) renderer
- Explicit ownership and no hidden globals
- Designed for multi-core and GPU-driven architectures

## Workspace Layout

```bash
rustgine/
├── Cargo.toml
├── crates/
│   ├── core/        # Shared types & engine glue
│   ├── ecs/         # Entity Component System
│   ├── scheduler/   # Parallel system scheduler
│   ├── render/      # WebGPU renderer
│   ├── platform/    # Windowing, input, time
│   ├── math/        # Math primitives
│   └── app/         # Main loop & application
└── examples/
```

## Getting Started

### Prerequisites

- Rust (latest stable, install from https://rustup.rs)

### Build and Run

Clone the repository and build the workspace:

```bash
git clone https://github.com/diamondbo/rustgine.git
cd rustgine
cargo build --workspace
```

To run the main application:

```bash
cargo run -p app
```

See [docs/architecture.md](docs/architecture.md) for a detailed architecture overview.
