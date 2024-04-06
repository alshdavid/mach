#!/bin/bash

export RUSTUP_HOME=$HOME/.local/rust/rustup
export CARGO_HOME=$HOME/.local/rust/cargo

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --no-modify-path -y

export PATH="$HOME/.local/rust/cargo/bin:$PATH"
echo "${HOME}/.local/rust/cargo/bin" >> $GITHUB_PATH
