#!/bin/sh
set -e

TARGET_PATH=$(node -e "const { dirname } = require('node:path'); console.log(dirname(dirname(require.resolve('@alshdavid/mach'))))")
BIN_PATH="$TARGET_PATH/mach/bin/mach"

if [ "$MACH_BINARY_PATH" != "" ]; then
  $MACH_BINARY_PATH $@
fi

$BIN_PATH $@
