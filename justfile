# Windows requires GNU coreuitls
set windows-shell := ["pwsh", "-NoLogo", "-NoProfileLoadTime", "-Command"]

libnode_version := "24.3.0"
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
  os + arch == "linuxamd64" { "x86_64-unknown-linux-musl" } \
else if \
  os + arch == "linuxarm64" { "aarch64-unknown-linux-musl" } \
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

bin_name := \
  if \
    os == "windows" { "nitropack_bundler_cli.exe" } \
  else \
    { "nitropack_bundler_cli" }

bin_name_out := \
  if \
    os == "windows" { "nitropack.exe" } \
  else \
    { "nitropack" }

cargo_base :=  join(justfile_directory(), "target", ".cargo", target, profile)
out_base :=  join(justfile_directory(), "target", os + "-" + arch)
out_dir :=  join(out_base, profile)
lib_dir := join(out_base, "lib")

build *ARGS:
  node ./scripts/build.mjs {{ARGS}}

[no-cd]
run *ARGS:
  just build
  {{out_dir}}/{{bin_name_out}} {{ARGS}}

test:
  cargo test

format arg="--check":
  #!/usr/bin/env bash
  just fmt {{arg}}
  just lint {{arg}}

fmt arg="--check":
  #!/usr/bin/env bash
  args=""
  while read -r line; do
    line=$(echo "$line" | tr -d "[:space:]")
    args="$args --config $line"
  done < "rust-fmt.toml"
  args=$(echo "$args" | xargs)
  if [ "{{arg}}" = "--fix" ]; then
    cargo fmt -- $args
  else
    cargo fmt --check -- $args
  fi

lint arg="--check":
  #!/usr/bin/env bash
  if [ "{{arg}}" = "--fix" ]; then
    cargo clippy --fix --allow-dirty -- --deny "warnings"
  else
    cargo clippy -- --deny "warnings"
  fi

bench:
  node ./scripts/bench.mjs


