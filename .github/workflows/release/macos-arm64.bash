#!/bin/bash
set -ev

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

NODE_VERSION=$(cat $ROOT_DIR/.nvmrc | sed -E '$ s/\n//g')
source $ROOT_DIR/.github/workflows/platform/install-nodejs.bash $NODE_VERSION
node -v

source $ROOT_DIR/.github/workflows/platform/install-rust.bash $NODE_VERSION
cargo -v