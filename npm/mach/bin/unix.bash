#!/bin/bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

if [ "$MACH_BIN_OVERRIDE" != "" ]; then
  "$MACH_BIN_OVERRIDE" $@
  exit
fi

ARCH=""
OS=""

case $(uname -m) in
  x86_64 | x86-64 | x64 | amd64)
    ARCH="amd64"
  ;;
  aarch64 | arm64)
    ARCH="arm64"
  ;;
esac

case $(uname -s) in
  Darwin)
    OS="macos"
  ;;
  Linux)
    OS="linux"
  ;;
esac

if [ "$OS$ARCH" != "" ]; then
  "$SCRIPT_DIR/mach-$OS-$ARCH/bin/mach"
else
  echo "Could not find Mach binary for your system. Please compile from source"
  echo "Override the built in binary by setting the \$MACH_BIN_OVERRIDE environment variable"
  exit 1
fi
