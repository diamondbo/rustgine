#!/usr/bin/env bash

# Resolve the project root (parent of scripts/)
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

# Path to root Cargo.toml
CARGO_TOML="$ROOT_DIR/Cargo.toml"

# Extract the version from Cargo.toml in the root
VERSION=$(sed -n '/\[workspace.package\]/,/^\[/{s/^version *= *"\(.*\)"/\1/p}' "$CARGO_TOML")

echo "$VERSION"
