#!/bin/bash
set -ev

# Just
export PATH="${HOME}/.local/just:$PATH"

# Rust
export RUSTUP_HOME="${HOME}/.local/rust/rustup"
export CARGO_HOME="${HOME}/.local/rust/cargo"
export PATH="${HOME}/.local/rust/cargo/bin:$PATH"

# Nodejs
export PATH="${HOME}/.local/nodejs/bin:$PATH"
export PATH="${HOME}/.local/nodejs/prefix/bin:$PATH"
export NPM_CONFIG_PREFIX="${HOME}/.local/nodejs/prefix"
pnpm config set store-dir $HOME/.local/nodejs/pnpm-store
