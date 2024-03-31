// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::PathBuf;

use deno_ast::ModuleSpecifier;
use deno_core::anyhow::anyhow;
use deno_core::error::AnyError;

use crate::deno_cli::util::path::is_banned_path_char;
use crate::deno_cli::util::path::path_with_stem_suffix;
use crate::deno_cli::util::path::root_url_to_safe_local_dirname;

/// Partitions the provided specifiers by the non-path and non-query parts of a specifier.
pub fn partition_by_root_specifiers<'a>(
  specifiers: impl Iterator<Item = &'a ModuleSpecifier>,
) -> BTreeMap<ModuleSpecifier, Vec<ModuleSpecifier>> {
  let mut root_specifiers: BTreeMap<ModuleSpecifier, Vec<ModuleSpecifier>> =
    Default::default();
  for remote_specifier in specifiers {
    let mut root_specifier = remote_specifier.clone();
    root_specifier.set_query(None);
    root_specifier.set_path("/");

    let specifiers = root_specifiers.entry(root_specifier).or_default();
    specifiers.push(remote_specifier.clone());
  }
  root_specifiers
}

/// Gets the directory name to use for the provided root.
pub fn dir_name_for_root(root: &ModuleSpecifier) -> PathBuf {
  root_url_to_safe_local_dirname(root)
}

/// Gets a unique file path given the provided file path
/// and the set of existing file paths. Inserts to the
/// set when finding a unique path.
pub fn get_unique_path(
  mut path: PathBuf,
  unique_set: &mut HashSet<String>,
) -> PathBuf {
  let original_path = path.clone();
  let mut count = 2;
  // case insensitive comparison so the output works on case insensitive file systems
  while !unique_set.insert(path.to_string_lossy().to_lowercase()) {
    path = path_with_stem_suffix(&original_path, &format!("_{count}"));
    count += 1;
  }
  path
}

pub fn make_url_relative(
  root: &ModuleSpecifier,
  url: &ModuleSpecifier,
) -> Result<String, AnyError> {
  root.make_relative(url).ok_or_else(|| {
    anyhow!(
      "Error making url ({}) relative to root: {}",
      url.to_string(),
      root.to_string()
    )
  })
}

pub fn is_remote_specifier(specifier: &ModuleSpecifier) -> bool {
  matches!(specifier.scheme().to_lowercase().as_str(), "http" | "https")
}

pub fn is_remote_specifier_text(text: &str) -> bool {
  let text = text.trim_start().to_lowercase();
  text.starts_with("http:") || text.starts_with("https:")
}

pub fn sanitize_filepath(text: &str) -> String {
  text
    .chars()
    .map(|c| if is_banned_path_char(c) { '_' } else { c })
    .collect()
}
