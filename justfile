set windows-shell := ["pwsh", "-NoLogo", "-NoProfileLoadTime", "-Command"]

MACH_VERSION := env_var_or_default("MACH_VERSION", "")
profile := env_var_or_default("profile", "debug")

os := env_var_or_default("os", os())

arch := \
if \
  env_var_or_default("arch", "") != "" { env_var("arch") } \
else if \
  arch() == "x86_64" { "amd64" } \
else if \
  arch() == "aarch64" { "arm64" } \
else \
  { arch() }

dylib := \
if \
  os == "windows" { "dll" } \
else if \
  os == "macos" { "dylib" } \
else if \
  os == "linux" { "so" } \
else \
  { os() }

bin := \
if \
  os == "windows" { "mach.exe" } \
else \
  { "mach" }

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

profile_cargo := \
if \
  profile != "debug" { "--profile " + profile } \
else \
  { "" }

target_cargo := \
if \
  target == "debug" { "" } \
else if \
  target == "" { "" } \
else \
  { "--target " + target } 

out_dir :=  join(justfile_directory(), "target", os + "-" + arch, profile)
out_dir_link :=  join(justfile_directory(), "target", profile)

_default:
  @echo "Available Env:"
  @echo "    profile"
  @echo "        debug [default]"
  @echo "        release"
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

[unix]
build:
  @# Install npm
  @test -d node_modules || npm install

  @# Clean dir
  @rm -rf "{{out_dir}}"
  @rm -rf "{{out_dir_link}}"
  @mkdir -p "{{out_dir}}"
  @rm -rf "./packages/mach_nodejs/_napi/index.node"
  
  @# Build crates
  cargo build {{profile_cargo}} {{target_cargo}}
  @cp "./target/.cargo/{{target}}/{{profile}}/libmach_bundler_npm_os_arch.{{dylib}}" "./packages/mach_nodejs/_napi/index.node"
  
  @# Copy binary
  @mkdir -p "{{out_dir}}/bin"
  @cp "./target/.cargo/{{target}}/{{profile}}/mach" "{{out_dir}}/bin"

[windows]
build:
  # Install npm
  if (!(Test-Path 'node_modules')) { npm install }

  # Clean dir
  @if (Test-Path {{out_dir}}) { Remove-Item -Recurse -Force {{out_dir}} | Out-Null }
  @if (Test-Path {{out_dir_link}}) { Remove-Item -Recurse -Force {{out_dir_link}} | Out-Null }
  @New-Item -ItemType "directory" -Force -Path "{{out_dir}}"  | Out-Null
  @if (Test-Path ".\packages\mach_nodejs\_napi\index.node") { Remove-Item -Recurse -Force ".\packages\mach_nodejs\_napi\index.node" | Out-Null }

  # Build mach and napi
  cargo build {{profile_cargo}} {{target_cargo}}
  Copy-Item ".\target\.cargo\{{target}}\{{profile}}\mach_bundler_npm_os_arch.{{dylib}}" -Destination ".\packages\mach_nodejs\_napi\index.node" | Out-Null  

  # Copy binary
  New-Item -ItemType "directory" -Force -Path "{{out_dir}}\bin" | Out-Null
  Copy-Item ".\target\.cargo\{{target}}\{{profile}}\mach.exe" -Destination "{{out_dir}}\bin" | Out-Null

build-tsc:
  cd "./packages/mach_nodejs" && npx tsc

[no-cd]
run *ARGS:
  just build
  pwd
  {{out_dir}}/bin/{{bin}} {{ARGS}}

serve:
  npx http-server -p 3000 ./examples

alias test-integration := integration-tests
integration-tests *ARGS:
  node --import ./node_modules/tsx/dist/loader.mjs ./testing/setup.ts {{ARGS}}

alias test-unit := unit-tests
unit-tests:
  cargo test

watch:
  npx nodemon

fmt:
  cargo +nightly fmt
  node_modules/.bin/prettier ./packages --write
  node_modules/.bin/prettier ./examples --write

bench-micro:
  cargo bench

[unix]
build-publish:
  npm i
  just build-publish-common
  just build
  just build-tsc
  cp "./README.md" "./packages/mach_npm"
  cp "./README.md" "./packages/mach_nodejs"

[windows]
build-publish:
  npm i 
  just build-publish-common
  just build
  just build-tsc
  Copy-Item ".\README.md" -Destination "packages\mach_npm" | Out-Null
  Copy-Item ".\README.md" -Destination "packages\mach_nodejs" | Out-Null

[private]
build-publish-common:
  node {{justfile_directory()}}/.github/scripts/ci/string-replace.mjs \
    "./packages/mach/Cargo.toml" \
    "0.0.0-local" \
    {{MACH_VERSION}}

  node {{justfile_directory()}}/.github/scripts/ci/string-replace.mjs \
    "./packages/mach_npm/package.json" \
    "0.0.0-local" \
    "{{MACH_VERSION}}"

  node {{justfile_directory()}}/.github/scripts/ci/json.mjs \
    "./packages/mach_nodejs/package.json" \
    "name" \
    "@alshdavid/mach-{{os}}-{{arch}}"

  node {{justfile_directory()}}/.github/scripts/ci/json.mjs \
    "./packages/mach_nodejs/package.json" \
    "version" \
    "{{MACH_VERSION}}"

  node {{justfile_directory()}}/.github/scripts/ci/json.mjs \
    "./packages/mach_nodejs/package.json" \
    "os.0" \
    $(node "{{justfile_directory()}}/.github/scripts/ci/map.mjs" "os" {{os}})

  node {{justfile_directory()}}/.github/scripts/ci/json.mjs \
    "./packages/mach_nodejs/package.json" \
    "cpu.0" \
    $(node "{{justfile_directory()}}/.github/scripts/ci/map.mjs" "arch" {{arch}})

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
