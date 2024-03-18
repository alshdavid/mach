use std::path::{Path, PathBuf};
use std::process::Stdio;

use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// Warning, very slow, do not use unless you have to
/// spawns a Node.js process and resolves a module
pub async fn native_resolve(
  from_path: &Path,
  specifier: &str,
) -> Result<PathBuf, String> {
  let mut command = Command::new("node");

  command.stderr(Stdio::null());
  command.stdout(Stdio::piped());
  command.stdin(Stdio::piped());
  command.current_dir(from_path);

  let mut child = command.spawn().unwrap();

  let mut stdin = child.stdin.take().unwrap();
  let script = format!("process.stdout.write(require.resolve('{}'))", specifier);
  stdin.write(script.as_bytes()).await.unwrap();
  stdin.flush().await.unwrap();
  drop(stdin);

  let output = child.wait_with_output().await.unwrap();
  if !output.status.success() {
    return Err("Unable to resolve specifier".to_string());
  }

  let Ok(result) = String::from_utf8(output.stdout) else {
    return Err("Unable to parse result".to_string());
  };

  return Ok(PathBuf::from(result));
}
