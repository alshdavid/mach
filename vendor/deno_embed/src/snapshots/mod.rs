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
