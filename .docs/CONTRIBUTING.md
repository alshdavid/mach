# Contributing

This project welcomes contributions, feel free to raise a PR.

## Dependencies

- Node.js 18+
- Deno
- PNPM
- Rust

## Building

```bash
just build
```

## Running Locally

```bash
# Will build /testing/fixtures/simple
just run build ./testing/fixtures/simple

# Alternatively
# Will build /testing/fixtures/simple
just fixture build simple
```

## Testing

```bash
just test
```

## Testing Locally

### Use an alias

```bash
# From root of repo
alias machd="${PWD}/target/debug/bin/mach"

machd version
```

### Add locally built binary to PATH

```bash
# From root of repo
export PATH="${PWD}/target/debug/bin:$PATH"

mach version
```
