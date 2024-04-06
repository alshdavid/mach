# Contributing

This project welcomes contributions, feel free to raise a PR.

## Dependencies

- Node.js 
  - [Recommend installing with fnm](https://github.com/Schniz/fnm)
- [PNPM](https://pnpm.io/installation)
- [Rust](https://rustup.rs/)

## Building

```bash
just build
```

## Running Locally

To run `mach {{...args}}` using the local repo you can use
```bash
just run {{...args}}
```

For example:

```bash
just run build ./testing/fixtures/simple
```

## Building fixtures

This will build the fixture under `./testing/fixtures/simple`

```bash
just fixture build simple
# just run build ./testing/fixtures/simple
```

## Testing

TODO no tests yet, will implement automated integration tests under `./testing` eventually

```bash
just test
```

## Using Locally Built Mach

### Use the helper scripts

```bash
# From root of repo
mkdir ~/.local
cp ./docs/scripts/mach-dev ~/.local
export PATH=~/.local/mach-dev:$PATH
export MACH_REPO_PATH="${PWD}"
```

Then add them to your `PATH`
```bash
# From root of repo
echo "" >> .zshrc
echo "# Mach Dev Variables" >> .zshrc
echo "export PATH=\"~/.local/mach-dev:\$PATH\"" >> .zshrc
echo "export MACH_REPO_PATH=\"${PWD}\"" >> .zshrc
```

Then use
```bash
# From anywhere on your computer
machd version # Local debug build
machr version # Local release build
```

### Use an alias

```bash
# From root of repo
alias machd="${PWD}/target/debug/bin/mach"

machd version
```
