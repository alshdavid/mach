#!/bin/bash

# Default to current latest
JUST_VERSION="$1"
if [ "$JUST_VERSION" = "" ]; then
  JUST_VERSION="1.25.2"
fi 

# Default to home directory
OUT_DIR="$2"
if [ "$OUT_DIR" = "" ]; then
  OUT_DIR="$HOME/.local/just"
fi 

URL=""
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

case "$OS-$ARCH" in
  linux-amd64)
    URL=https://github.com/casey/just/releases/download/${JUST_VERSION}/just-${JUST_VERSION}-x86_64-unknown-linux-musl.tar.gz
  ;;
  linux-arm64)
    URL=https://github.com/casey/just/releases/download/${JUST_VERSION}/just-${JUST_VERSION}-aarch64-unknown-linux-musl.tar.gz
  ;;
  macos-amd64)
    URL=https://github.com/casey/just/releases/download/${JUST_VERSION}/just-${JUST_VERSION}-x86_64-apple-darwin.tar.gz
  ;;
  macos-arm64)
    URL=https://github.com/casey/just/releases/download/${JUST_VERSION}/just-${JUST_VERSION}-aarch64-apple-darwin.tar.gz
  ;;
esac

if [ "$URL" == "" ]; then
  echo "Cannot find installer for Nodejs"
  exit 1
fi

echo $URL

test -d $OUT_DIR && rm -rf $OUT_DIR
curl -s -L --url $URL | tar -xzf - -C $OUT_DIR

export PATH="${OUT_DIR}:$PATH"
echo "${OUT_DIR}" >> $GITHUB_PATH
