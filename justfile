# Env Variables
BIN_VERSION := env_var_or_default("BIN_VERSION", "")
profile := env_var_or_default("profile", "debug")
target := env_var_or_default("target", "")

_default:
  @echo "Available Env:"
  @echo "    profile"
  @echo "        debug [default]"
  @echo "        release"
  @echo "        release-lto"
  @echo "    target"
  @echo "        auto [default]"
  @echo "        x86_64-unknown-linux-gnu"
  @echo "        aarch64-unknown-linux-gnu"
  @echo "        x86_64-apple-darwin"
  @echo "        aarch64-apple-darwin"
  @echo "        x86_64-pc-windows-msvc"
  @echo "        aarch64-pc-windows-msvc"
  @just --list --unsorted

build:
  rm -rf ./target/{{profile}}
  just _create_out_dir
  cargo build \
    {{ if profile != "debug" { "--profile " + profile } else { "" } }} \
    {{ if target != "" { "--target " + target } else { "" } }}
  just _copy_cargo
  just _build_adapters

_build_adapters:
  #!/usr/bin/env sh
  set -ev
  cd adapters
  for file in `ls .`; do 
    cd $file
    just build
    cd ..
  done

run *ARGS:
  just build
  ./target/{{profile}}/bin/mach {{ARGS}}

# serve:
#   npx http-server -p 3000 ./testing/fixtures

# test:
#   cargo test

# fixture cmd fixture *ARGS:
#   @just build
#   ./target/{{profile}}/bin/mach {{cmd}} {{ARGS}} ./testing/fixtures/{{fixture}}

# fmt:
#   cargo +nightly fmt

# three-js:
#   node ./.github/scripts/ci/benchmark.mjs

@_create_out_dir:
  mkdir -p ./target/{{profile}}
  mkdir -p ./target/{{profile}}/bin

@_copy_cargo:
  cp ./target/.cargo/{{profile}}/mach ./target/{{profile}}/bin

# @_build_npm:
#   @just {{ if `node .github/scripts/ci/package-sha.mjs read` == "true" { "_build_npm_actions" } else { "_skip" } }}

# @_build_npm_actions:
#   echo building npm packages
#   pnpm install
#   cd npm/node-adapter && pnpm run build
#   cp -r npm/node-adapter/lib ./target/{{profile}}/lib/node-adapter
#   cp -r npm/node-adapter/types ./target/{{profile}}/lib/node-adapter
#   node .github/scripts/ci/package-sha.mjs set

# @_skip:
#   echo skip
