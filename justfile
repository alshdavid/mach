set windows-shell := ["pwsh", "-NoLogo", "-NoProfileLoadTime", "-Command"]

MACH_VERSION := env_var_or_default("MACH_VERSION", "")
profile := env_var_or_default("profile", "debug")

os := \
if \
  env_var_or_default("os", "") == "Windows_NT" { "windows" } \
else if \
  env_var_or_default("os", "") != "" { env_var("os") } \
else \
  { os() }

arch := \
if \
  env_var_or_default("arch", "") != "" { env_var("arch") } \
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
  @rm -rf {{out_dir}}
  @rm -rf {{out_dir_link}}
  cargo build {{profile_cargo}} {{target_cargo}}
  @mkdir -p {{out_dir}}
  @mkdir -p {{out_dir}}/bin
  @cp ./target/.cargo/{{target}}/{{profile}}/mach {{out_dir}}/bin
  @ln -s {{out_dir}} {{out_dir_link}}
  @rm -rf npm/mach/cmd
  @cp -r {{out_dir}} npm/mach/cmd
  @mv npm/mach/cmd/bin/mach npm/mach/cmd/bin/mach.exe
  test -d node_modules || pnpm install
  cd ./mach-nodejs && \
    npx cargo-cp-artifact -nc ./lib/napi/index.node -- \
    cargo build --message-format=json-render-diagnostics {{profile_cargo}} {{target_cargo}}
  @cp -r ./mach-nodejs/lib {{out_dir}}/nodejs

[windows]
build:
  @if (Test-Path {{out_dir}}) { Remove-Item -Recurse -Force {{out_dir}} | Out-Null }
  @if (Test-Path {{out_dir_link}}) { Remove-Item -Recurse -Force {{out_dir_link}} | Out-Null }
  cargo build {{profile_cargo}} {{target_cargo}}
  @New-Item -ItemType "directory" -Force -Path "{{out_dir}}"  | Out-Null| Out-Null
  @New-Item -ItemType "directory" -Force -Path "{{out_dir}}\bin" | Out-Null
  @Copy-Item ".\target\.cargo\{{target}}\{{profile}}\mach.exe" -Destination "{{out_dir}}\bin" | Out-Null  @New-Item -ItemType SymbolicLink -Path "{{out_dir_link}}" -Target "{{out_dir}}" | Out-Null
  @if (Test-Path "npm\mach\cmd") { Remove-Item -Recurse -Force "npm\mach\cmd" | Out-Null }
  @Copy-Item "{{out_dir}}" -Destination "npm\mach\cmd" -Recurse | Out-Null
  if (!(Test-Path 'node_modules')) { pnpm install }
  cd .\mach-nodejs && \
    npx cargo-cp-artifact -nc ./lib/napi/index.node -- \
    cargo build --message-format=json-render-diagnostics {{profile_cargo}} {{target_cargo}}
  Copy-Item ".\mach-nodejs\lib" -Destination "{{out_dir}}\nodejs" -Recurse | Out-Null

[unix]
run *ARGS:
  just build
  {{out_dir}}/bin/mach {{ARGS}}

[windows]
run *ARGS:
  just build
  {{out_dir}}/bin/mach.exe {{ARGS}}

[unix]
example cmd fixture *ARGS:
  @just build
  {{out_dir}}/bin/mach {{cmd}} {{ARGS}} ./examples/{{fixture}}

[windows]
example cmd fixture *ARGS:
  @just build
  {{out_dir}}/bin/mach.exe {{cmd}} {{ARGS}} ./examples/{{fixture}}

serve:
  npx http-server -p 3000 ./examples

test:
  cargo test

fmt:
  cargo +nightly fmt
  ./.github/scripts/node_modules/.bin/prettier ./mach-nodejs --write
  ./.github/scripts/node_modules/.bin/prettier ./npm --write
  ./.github/scripts/node_modules/.bin/prettier ./examples --write

[unix]
build-publish:
  just build
  just build-publish-common
  cp -r {{out_dir}}/* "npm/mach-os-arch"
  mv "npm/mach-os-arch/bin/mach" "npm/mach-os-arch/bin/mach.exe"
  rm -rf "npm/mach/cmd"
  mkdir -p "npm/mach/cmd/bin"
  touch "npm/mach/cmd/bin/mach.exe"
  cp "./README.md" "npm/mach"

[windows]
build-publish:
  just build
  just build-publish-common
  Copy-Item {{out_dir}}\* -Destination "npm\mach-os-arch" -Recurse | Out-Null
  Remove-Item -Recurse -Force "npm\mach\cmd" | Out-Null
  New-Item -ItemType "directory" -Force -Path "npm\mach\cmd\bin" | Out-Null
  New-Item -ItemType "file" "npm\mach\cmd\bin\mach.exe"
  Copy-Item ".\README.md" -Destination "npm\mach" | Out-Null

[private]
build-publish-common:
  node {{justfile_directory()}}/.github/scripts/ci/string-replace.mjs \
    "./mach/Cargo.toml" \
    "0.0.0-local" \
    {{MACH_VERSION}}

  node {{justfile_directory()}}/.github/scripts/ci/string-replace.mjs \
    "./npm/mach/package.json" \
    "0.0.0-local" \
    "{{MACH_VERSION}}"

  node {{justfile_directory()}}/.github/scripts/ci/json.mjs \
    "./npm/mach-os-arch/package.json" \
    "name" \
    "@alshdavid/mach-{{os}}-{{arch}}"

  node {{justfile_directory()}}/.github/scripts/ci/json.mjs \
    "./npm/mach-os-arch/package.json" \
    "version" \
    "{{MACH_VERSION}}"

  node {{justfile_directory()}}/.github/scripts/ci/json.mjs \
    "./npm/mach-os-arch/package.json" \
    "os.0" \
    $(node "{{justfile_directory()}}/.github/scripts/ci/map.mjs" "os" {{os}})

  node {{justfile_directory()}}/.github/scripts/ci/json.mjs \
    "./npm/mach-os-arch/package.json" \
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
