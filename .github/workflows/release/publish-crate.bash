#!/bin/bash
set -e

JOB_NAME="linux-amd64"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

source $ROOT_DIR/.github/workflows/platform/unix/setup.bash

if [ "$BRANCH_NAME" == "main" ]; then
  just build-publish
  cargo publish --allow-dirty --package mach_bundler_core --token $CRATES_IO_API_TOKEN
fi