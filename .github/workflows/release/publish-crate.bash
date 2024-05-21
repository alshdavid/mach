#!/bin/bash
set -e

JOB_NAME="linux-amd64"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

if [ "$BRANCH_NAME" == "main" ]; then
  cargo publish --allow-dirty --package mach_bundler_core --token $CRATES_IO_API_TOKEN
fi