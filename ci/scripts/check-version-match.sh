#!/usr/bin/env bash
set -euo pipefail

# Resolve project root
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

# Paths
CARGO_TOML="$ROOT_DIR/Cargo.toml"
CHANGELOG="$ROOT_DIR/CHANGELOG.md"
VERSION_YML_PATH="$ROOT_DIR/crates/version.yml"

# --- Parse version from Cargo.toml ---
VERSION_CARGO=$(sed -n '/\[workspace.package\]/,/^\[/{s/^version *= *"\(.*\)"/\1/p}' "$CARGO_TOML" | tr -d '\r')

if [[ -z "$VERSION_CARGO" ]]; then
  echo "❌ Could not parse version from Cargo.toml!"
  exit 1
fi

echo "Cargo.toml version: $VERSION_CARGO"

# --- Parse latest version from CHANGELOG.md ---
# Assumes changelog headings like: ## [0.2.0] - yyyy-mm-dd
VERSION_CHANGELOG=$(grep -E '^## \[[0-9]+\.[0-9]+\.[0-9]+\]' "$CHANGELOG" \
    | head -n1 \
    | sed -E 's/^## \[([0-9]+\.[0-9]+\.[0-9]+)\].*/\1/' \
    | tr -d '\r')

if [[ -z "$VERSION_CHANGELOG" ]]; then
  echo "❌ Could not parse latest version from CHANGELOG.md!"
  exit 1
fi

echo "CHANGELOG.md latest version: $VERSION_CHANGELOG"

# --- Compare versions ---
if [[ "$VERSION_CARGO" == "$VERSION_CHANGELOG" ]]; then
  echo "✅ Cargo.toml and CHANGELOG.md versions are in sync."
elif [[ "$VERSION_CARGO" != "$VERSION_CHANGELOG" ]]; then
  echo "❌ Version mismatch: Cargo.toml ($VERSION_CARGO) vs CHANGELOG.md ($VERSION_CHANGELOG)"
  exit 1
fi

# --- Write version.yml ---
mkdir -p "$(dirname "$VERSION_YML_PATH")"
echo "version: $VERSION_CARGO" > "$VERSION_YML_PATH"
