use std::path::PathBuf;

use sha2::Digest;
use sha2::Sha256;

pub fn hash_sha_256(input: &[u8]) -> String {
  let mut hasher = <Sha256 as Digest>::new();
  hasher.update(input);
  let digest = hasher.finalize();
  return format!("{:x}", digest);
}

pub fn hash_string_sha_256(input: &str) -> String {
  return hash_sha_256(input.as_bytes());
}

pub fn hash_path_buff_sha_256(input: &PathBuf) -> String {
  return hash_string_sha_256(input.to_str().unwrap());
}
