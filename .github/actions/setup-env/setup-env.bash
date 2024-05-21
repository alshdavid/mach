#!/bin/bash
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $SCRIPT_DIR))

cd $ROOT_DIR

# Just
export PATH="${HOME}/.local/just:$PATH"

# Rust
export RUSTUP_HOME="${HOME}/.local/rust/rustup"
export CARGO_HOME="${HOME}/.local/rust/cargo"
export PATH="${HOME}/.local/rust/cargo/bin:$PATH"

# Nodejs
export PATH="${HOME}/.local/nodejs/bin:$PATH"
export PATH="${HOME}/.local/nodejs/prefix/bin:$PATH"
export NPM_CONFIG_PREFIX="${HOME}/.local/nodejs/prefix"
pnpm config set store-dir $HOME/.local/nodejs/pnpm-store

ENV_PATH="${ROOT_DIR}/.env"

if [ ! -f "$ENV_PATH" ]; then
  echo "missing ${ENV_PATH}"
  exit 1
fi

echo "Reading $ENV_PATH"
while read -r LINE; do
  # Remove leading and trailing whitespaces, and carriage return
  CLEANED_LINE=$(echo "$LINE" | awk '{$1=$1};1' | tr -d '\r')

  if [[ $CLEANED_LINE != '#'* ]] && [[ $CLEANED_LINE == *'='* ]]; then
    export "$CLEANED_LINE"
  fi
done < "$ENV_PATH"
