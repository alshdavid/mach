#!/bin/sh

if [ "$MACH_BINARY_PATH" != "" ]; then
  $MACH_BINARY_PATH $@
else
  TARGET_PATH=$(node -e "console.log(require('path').dirname(require.resolve('@alshdavid/mach')))")
  $TARGET_PATH/mach/bin/mach $@
fi
