BIN_VERSION := env_var_or_default("BIN_VERSION", "")
NPM_VERSION := env_var_or_default("NPM_VERSION", "")
NPM_BIN_TARGET := env_var_or_default("NPM_BIN_TARGET", "")

profile := env_var_or_default("profile", "debug")

os := env_var_or_default("os", os())
arch := \
if \
  env_var_or_default("arch", "") != "" { env_var_or_default("arch", "") } \
else if \
  arch() == "x86_64" { "amd64" } \
else if \
  arch() == "aarch64" { "arm64" } \
else \
  { arch() }

target := \
if \
  os + arch == "linuxamd64" { "x86_64-unknown-linux-gnu" } \
else if \
  os + arch == "linuxarm64" { "aarch64-unknown-linux-gnu" } \
else if \
  os + arch == "macosamd64" { "x86_64-apple-darwin" } \
else if\
  os + arch == "macosarm64" { "aarch64-apple-darwin" } \
else if \
  os + arch == "windowsamd64" { "x86_64-pc-windows-msvc" } \
else if \
  os + arch == "windowsarm64" { "aarch64-pc-windows-msvc" } \
else \
  { env_var_or_default("target", "debug") }

profile_cargo := if profile != "debug" { "--profile " + profile } else { "" }

target_cargo := \
if \
target == "debug" \
  { "" } \
else if \
target == "" \
  { "" } \
else \
  { "--target " + target } 

out_dir := "./target/" + os + "_" + arch + "_" + profile

_default:
  @echo "Available Env:"
  @echo "    profile"
  @echo "        debug [default]"
  @echo "        release"
  @echo "        release-lto"
  @echo "    os"
  @echo "        auto [default]"
  @echo "        linux"
  @echo "        macos"
  @echo "        windows"
  @echo "    arch"
  @echo "        auto [default]"
  @echo "        arm64"
  @echo "        amd64"
  @just --list --unsorted

build:
  @just _build_npm
  cargo build {{profile_cargo}} {{target_cargo}}
  @just _create_out_dir
  @just _copy_cargo
  @just _copy_npm

run *ARGS:
  @just build
  {{out_dir}}/bin/mach {{ARGS}}

serve:
  npx http-server -p 3000 ./testing/fixtures

test:
  cargo test

fixture cmd fixture *ARGS:
  @just build
  {{out_dir}}/bin/mach {{cmd}} {{ARGS}} ./testing/fixtures/{{fixture}}

fmt:
  cargo +nightly fmt

benchmark project="mach" count="50" script="build" *ARGS="":
  @just {{ if project == "mach" { "build" } else { "_skip" } }}
  just benchmark-generate {{project}} {{count}}
  cd benchmarks/{{project}}_{{count}} && \
  rm -rf dist && \
  CMD="console.log(require(\"./package.json\").scripts[\"build\"])" && \
  CMD="$(echo $CMD | node)" && \
  echo $CMD && \
  mach_profiler=../{{project}}_{{count}}.csv \
  time bash -c "$CMD {{ARGS}}"

benchmark-generate project="mach" count="50":
  PROJECT={{project}} \
  BENCH_COPIES={{count}} \
  node .github/scripts/ci/generate_benchmark.mjs

@_create_out_dir:
  rm -rf {{out_dir}}
  mkdir -p {{out_dir}}
  mkdir -p {{out_dir}}/bin
  mkdir -p {{out_dir}}/lib
  mkdir -p {{out_dir}}/lib/node-adapter

@_copy_cargo:
  cp ./target/.cargo/{{target}}/{{profile}}/mach {{out_dir}}/bin

@_build_npm:
  @just {{ if `node .github/scripts/ci/package-sha.mjs read` == "true" { "_build_npm_actions" } else { "_skip" } }}

@_build_npm_internal_actions:
  echo building npm packages
  pnpm install
  cd npm/node-adapter && pnpm run build
  node .github/scripts/ci/package-sha.mjs set

@_copy_npm:
  cp -r npm/node-adapter/lib {{out_dir}}/lib/node-adapter
  cp -r npm/node-adapter/types {{out_dir}}/lib/node-adapter

_skip:
  echo "skip"