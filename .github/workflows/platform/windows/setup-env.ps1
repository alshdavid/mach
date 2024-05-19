#!/bin/bash
set -e

# Just
$env:Path = $HOME + '\.local\just;' + $env:Path

# Rust
$env:RUSTUP_HOME = "${HOME}\.local\rust\rustup"
$env:CARGO_HOME = "${HOME}\.local\rust\cargo"
$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path

# Nodejs
$env:Path = $HOME + '\.local\nodejs;' + $env:Path
$env:Path = $HOME + '\.local\nodejs\prefix\bin;' + $env:Path
$env:NPM_CONFIG_PREFIX = $HOME + '\.local\nodejs\prefix'
pnpm config set store-dir $HOME/.local/nodejs/pnpm-store
