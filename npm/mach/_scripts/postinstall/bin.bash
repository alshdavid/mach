#!/bin/sh
set -e

TARGET_PATH=$(node -e "const { dirname } = require('node:path'); console.log(dirname(dirname(require.resolve('@alshdavid/mach'))))")
BIN_PATH="$TARGET_PATH/mach/bin/mach"

if [ "$MACH_BINARY_PATH" != "" ]; then
  $MACH_BINARY_PATH $@
fi

if [[ "$@" = *"--version"* ]]; then
  node -e "console.log('NPM Package:', JSON.parse(require('node:fs').readFileSync(require('node:path').join('${TARGET_PATH}', 'package.json'))).version)"
  echo "Mach Binary: $($BIN_PATH --version)"
else
  $BIN_PATH $@
fi
