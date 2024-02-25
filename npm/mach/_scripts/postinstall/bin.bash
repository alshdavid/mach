#!/bin/sh

if [ "$MACH_BINARY_PATH" != "" ]; then
  $MACH_BINARY_PATH $@
else
  TARGET_PATH=$(node -e "const { dirname } = require('path'); console.log(dirname(dirname(require.resolve('@alshdavid/mach'))))")
  $TARGET_PATH/mach/bin/mach $@
fi