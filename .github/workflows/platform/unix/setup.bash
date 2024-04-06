#!/bin/bash
set -ev

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

# Install Node.js
NODE_VERSION=$(cat $ROOT_DIR/.nvmrc | sed -E '$ s/\n//g')
source $SCRIPT_DIR/install-nodejs.bash $NODE_VERSION
node -v

# Install Rust
source $SCRIPT_DIR/install-rust.bash
cargo --version

# Install Just
source $SCRIPT_DIR/install-just.bash
just --version
