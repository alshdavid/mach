# Env Variables
BIN_VERSION := env_var_or_default("BIN_VERSION", "")
NPM_VERSION := env_var_or_default("NPM_VERSION", "")
NPM_BIN_TARGET := env_var_or_default("NPM_BIN_TARGET", "")

profile := env_var_or_default("profile", "debug")
profile_cargo := if profile != "debug" { "--profile " + profile } else { "" }

target := env_var_or_default("target", "")
target_cargo := if target != "" { "--target " + target } else { "" }

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
  @just _create_out_dir
  @just _build_npm
  cargo build {{profile_cargo}} {{target_cargo}}
  @just _copy_cargo

run *ARGS:
  @just build
  ./target/{{profile}}/bin/mach {{ARGS}}

serve:
  npx http-server -p 3000 ./testing/fixtures

test:
  cargo test

fixture cmd fixture *ARGS:
  @just build
  ./target/{{profile}}/bin/mach {{cmd}} {{ARGS}} ./testing/fixtures/{{fixture}}

fmt:
  cargo +nightly fmt

@_create_out_dir:
  rm -rf ./target/{{profile}}
  mkdir -p ./target/{{profile}}
  mkdir -p ./target/{{profile}}/bin
  mkdir -p ./target/{{profile}}/lib
  mkdir -p ./target/{{profile}}/lib/node-adapter

@_copy_cargo:
  cp ./target/.cargo/{{profile}}/mach ./target/{{profile}}/bin

@_build_npm:
  {{ if `node .github/scripts/ci/package-sha.mjs read` == "true" { "cd npm/node-adapter && pnpm run build && node ../../.github/scripts/ci/package-sha.mjs set" } else { "echo skip npm build" } }}
  cp -r npm/node-adapter/lib ./target/{{profile}}/lib/node-adapter
  cp -r npm/node-adapter/types ./target/{{profile}}/lib/node-adapter
