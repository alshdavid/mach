export const OS =
  process.env.os ||
  {
    darwin: "macos",
    linux: "linux",
    win32: "windows",
  }[process.platform];

export const ARCH =
  process.env.arch ||
  {
    arm64: "arm64",
    x64: "amd64",
  }[process.arch];

export const OS_ARCH = `${OS}-${ARCH}`;

export const PROFILE = process.env.profile || "debug";

export const CARGO_PROFILE = PROFILE !== "debug" ? PROFILE : "dev";

export const CARGO_TARGET = {
  "linux-amd64": "x86_64-unknown-linux-musl",
  "linux-arm64": "aarch64-unknown-linux-musl",
  "macos-amd64": "x86_64-apple-darwin",
  "macos-arm64": "aarch64-apple-darwin",
  "windows-amd64": "x86_64-pc-windows-msvc",
  "windows-arm64": "aarch64-pc-windows-msvc",
}[`${OS}-${ARCH}`];

export const CARGO_BIN_NAME =
  OS === "windows" ? "mach_bundler_cli.exe" : "mach_bundler_cli";

export const BIN_NAME = OS === "windows" ? "mach.exe" : "mach";
