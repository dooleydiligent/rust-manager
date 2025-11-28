#!/bin/bash
# Podman Dev Container Post-Create Hook for Dioxus Fullstack
# This script runs automatically after the container is created.
# It installs Rust development tools, Dioxus CLI, and Tailwind CSS.
# Compatible with Podman rootless and root modes.

set -e
export DEBIAN_FRONTEND=noninteractive

echo "=== Updating package lists ==="
apt-get update -qq

echo "=== Installing minimal dependencies for Dioxus development ==="
apt-get install -y --no-install-recommends \
  ca-certificates \
  pkg-config \
  curl \
  procps \
  libvirt-dev \
  build-essential \
  npm \
  nodejs
echo "=== Installing wasm-pack ==="
cargo install -f wasm-pack

echo "=== Installing cargo-binstall ==="
cargo install cargo-binstall --quiet

echo "=== Installing dioxus-cli ==="
cargo binstall dioxus-cli -y

echo "=== Pre-fetching cargo dependencies ==="
cargo fetch --quiet

rustup component add rustfmt # rust-src  clippy

echo "=== Installing Tailwind CSS ==="
# Ensure node is available. Prefer nvm if installed, otherwise fall back to apt's nodejs/npm
if ! command -v node >/dev/null 2>&1; then
  export NVM_DIR="$HOME/.nvm"
  [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
  if command -v nvm >/dev/null 2>&1; then
    nvm install node || true
  else
    apt update -y && apt install -y nodejs npm || true
  fi
fi

# Install Tailwind and its CLI locally in the workspace so the CLI can always find the package
npm install --no-audit --no-fund @tailwindcss/cli tailwindcss --prefix . || true

echo "=== Generating Tailwind CSS ==="
if [ -f ./input.css ]; then
if [ -x ./node_modules/.bin/tailwindcss ]; then
  ./node_modules/.bin/tailwindcss -i ./input.css -o ./assets/tailwind.css
else
  # fallback to npx (global CLI) if present
  npx tailwindcss -i ./input.css -o ./assets/tailwind.css || true
fi
  echo "✓ Tailwind CSS generated"
else
  echo "⚠ input.css not found, skipping Tailwind generation"
fi

echo "building"
cargo build 
echo ""
echo "=== Setup complete ==="
echo "Container ready for Dioxus development!"
echo ""
echo "Available commands:"
echo "  dx serve --platform web      — Start dev server with live reload"
echo "  dx build --platform web      — Build for web"
echo "  cargo test                   — Run Rust tests"
echo "  npm run build:css            — Build Tailwind CSS"
