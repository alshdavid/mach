// Prebuilt Deno v8 snapshots
// https://github.com/alshdavid/deno_embed

#[cfg(target_os = "linux")]
#[cfg(target_arch = "x86_64")]
pub const SNAPSHOT: &[u8] = include_bytes!("./v8-deno-linux-amd64.bin");

#[cfg(target_os = "linux")]
#[cfg(target_arch = "aarch64")]
pub const SNAPSHOT: &[u8] = include_bytes!("./v8-deno-linux-arm64.bin");

#[cfg(target_os = "macos")]
#[cfg(target_arch = "x86_64")]
pub const SNAPSHOT: &[u8] = include_bytes!("./v8-deno-macos-amd64.bin");

#[cfg(target_os = "macos")]
#[cfg(target_arch = "aarch64")]
pub const SNAPSHOT: &[u8] = include_bytes!("./v8-deno-macos-arm64.bin");

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
pub const SNAPSHOT: &[u8] = include_bytes!("./v8-deno-windows-amd64.bin");

/// https://github.com/denoland/deno/tree/v1.41.1
#[allow(dead_code)]
pub const DENO_VERSION: &str = "1.41.1";
