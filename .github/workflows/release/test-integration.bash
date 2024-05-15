#!/bin/bash
set -ev

JOB_NAME="linux-amd64"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

source $ROOT_DIR/.github/workflows/platform/unix/setup.bash

sudo apt-get update
rustup target add x86_64-unknown-linux-gnu
export MACH_SKIP_POST_INSTALL="true"

profile=release os=linux arch=amd64 just build

just test-integration