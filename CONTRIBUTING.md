# Setup

## Getting Started

Install the following dependencies:

- [Just](https://github.com/casey/just)
- [Nodejs](https://nodejs.org)
- [Rust](https://www.rust-lang.org/)
- Chrome or Chromium - for integration tests

## Building an Example

To build an example project with Nitropack, simply navigate to the example to build and run:

```bash
cd examples/basic
just run build

# Equivalent to:
#   cd examples/basic
#   nitropack build
```
In this case `just run` is the equivalent of `nitropack` and the command will be run from the directory of the current shell.

## Building Nitropack

Nitropack compiles to a binary and a native Nodejs addon accessible via the `@alshdavid/nitropack` npm package.

To build Nitropack, run:

```bash
just build
```

For more options:

```bash
profile=release just build
```

## Tests

### Unit Tests

```bash
just test-unit
```
### Integration Tests

```bash
# Optional, will try to determine bin path automatically
export CHROME_EXECUTABLE_PATH="/usr/bin/google-chrome-stable"

just test-integration
```