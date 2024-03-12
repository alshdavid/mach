// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

pub const GIT_COMMIT_HASH: &str = "6d062cf0c";
pub const TYPESCRIPT: &str = "5.3.3";
pub const DENO_VERSION: &str = "1.42.1";

pub fn deno() -> &'static str {
  DENO_VERSION
}

// Keep this in sync with `deno()` above
pub fn get_user_agent() -> &'static str {
  "Deno/1.42.1"
}

pub fn is_canary() -> bool {
  false
}

pub fn release_version_or_canary_commit_hash() -> &'static str {
  DENO_VERSION
}
