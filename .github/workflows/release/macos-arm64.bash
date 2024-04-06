#!/bin/bash
set -ev

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

# Install Node.js
NODE_VERSION=$(cat $ROOT_DIR/.nvmrc | sed -E '$ s/\n//g')
source $ROOT_DIR/.github/workflows/platform/unix/install-nodejs.bash $NODE_VERSION
node -v

# Install Rust
source $ROOT_DIR/.github/workflows/platform/unix/install-rust.bash
cargo -version

# Install Just
source $ROOT_DIR/.github/workflows/platform/unix/install-rust.bash
just --version
