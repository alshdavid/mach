// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use crate::deno_cli::cache::LocalLspHttpCache;

use deno_ast::MediaType;
use deno_core::error::AnyError;
use deno_core::parking_lot::Mutex;
use deno_core::url::Position;
use deno_core::url::Url;
use deno_core::ModuleSpecifier;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;

/// Used in situations where a default URL needs to be used where otherwise a
/// panic is undesired.
pub static INVALID_SPECIFIER: Lazy<ModuleSpecifier> =
  Lazy::new(|| ModuleSpecifier::parse("deno://invalid").unwrap());

/// Matches the `encodeURIComponent()` encoding from JavaScript, which matches
/// the component percent encoding set.
///
/// See: <https://url.spec.whatwg.org/#component-percent-encode-set>
const COMPONENT: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
  .add(b' ')
  .add(b'"')
  .add(b'#')
  .add(b'<')
  .add(b'>')
  .add(b'?')
  .add(b'`')
  .add(b'{')
  .add(b'}')
  .add(b'/')
  .add(b':')
  .add(b';')
  .add(b'=')
  .add(b'@')
  .add(b'[')
  .add(b'\\')
  .add(b']')
  .add(b'^')
  .add(b'|')
  .add(b'$')
  .add(b'%')
  .add(b'&')
  .add(b'+')
  .add(b',');

fn hash_data_specifier(specifier: &ModuleSpecifier) -> String {
  let mut file_name_str = specifier.path().to_string();
  if let Some(query) = specifier.query() {
    file_name_str.push('?');
    file_name_str.push_str(query);
  }
  crate::deno_cli::util::checksum::gen(&[file_name_str.as_bytes()])
}

fn to_deno_url(specifier: &Url) -> String {
  let mut string = String::with_capacity(specifier.as_str().len() + 6);
  string.push_str("deno:/");
  string.push_str(specifier.scheme());
  for p in specifier[Position::BeforeHost..].split('/') {
    string.push('/');
    string.push_str(
      &percent_encoding::utf8_percent_encode(p, COMPONENT).to_string(),
    );
  }
  string
}

fn from_deno_url(url: &Url) -> Option<Url> {
  if url.scheme() != "deno" {
    return None;
  }
  let mut segments = url.path_segments()?;
  let mut string = String::with_capacity(url.as_str().len());
  string.push_str(segments.next()?);
  string.push_str("://");
  string.push_str(
    &percent_encoding::percent_decode(segments.next()?.as_bytes())
      .decode_utf8()
      .ok()?,
  );
  for segment in segments {
    string.push('/');
    string.push_str(
      &percent_encoding::percent_decode(segment.as_bytes())
        .decode_utf8()
        .ok()?,
    );
  }
  Url::parse(&string).ok()
}

/// This exists to make it a little bit harder to accidentally use a `Url`
/// in the wrong place where a client url should be used.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct LspClientUrl(Url);

impl LspClientUrl {
  pub fn new(url: Url) -> Self {
    Self(url)
  }

  pub fn as_url(&self) -> &Url {
    &self.0
  }

  pub fn into_url(self) -> Url {
    self.0
  }

  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }
}

impl std::fmt::Display for LspClientUrl {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

#[derive(Debug, Default)]
struct LspUrlMapInner {
  specifier_to_url: HashMap<ModuleSpecifier, LspClientUrl>,
  url_to_specifier: HashMap<Url, ModuleSpecifier>,
}

impl LspUrlMapInner {
  fn put(&mut self, specifier: ModuleSpecifier, url: LspClientUrl) {
    self
      .url_to_specifier
      .insert(url.as_url().clone(), specifier.clone());
    self.specifier_to_url.insert(specifier, url);
  }

  fn get_url(&self, specifier: &ModuleSpecifier) -> Option<&LspClientUrl> {
    self.specifier_to_url.get(specifier)
  }

  fn get_specifier(&self, url: &Url) -> Option<&ModuleSpecifier> {
    self.url_to_specifier.get(url)
  }
}

#[derive(Debug, Clone, Copy)]
pub enum LspUrlKind {
  File,
  Folder,
}

/// A bi-directional map of URLs sent to the LSP client and internal module
/// specifiers. We need to map internal specifiers into `deno:` schema URLs
/// to allow the Deno language server to manage these as virtual documents.
#[derive(Debug, Default, Clone)]
pub struct LspUrlMap {
  local_http_cache: Option<Arc<LocalLspHttpCache>>,
  inner: Arc<Mutex<LspUrlMapInner>>,
}

impl LspUrlMap {
  pub fn set_cache(&mut self, http_cache: Option<Arc<LocalLspHttpCache>>) {
    self.local_http_cache = http_cache;
  }

  /// Normalize a specifier that is used internally within Deno (or tsc) to a
  /// URL that can be handled as a "virtual" document by an LSP client.
  pub fn normalize_specifier(
    &self,
    specifier: &ModuleSpecifier,
  ) -> Result<LspClientUrl, AnyError> {
    if let Some(cache) = &self.local_http_cache {
      if matches!(specifier.scheme(), "http" | "https") {
        if let Some(file_url) = cache.get_file_url(specifier) {
          return Ok(LspClientUrl(file_url));
        }
      }
    }
    let mut inner = self.inner.lock();
    if let Some(url) = inner.get_url(specifier).cloned() {
      Ok(url)
    } else {
      let url = if specifier.scheme() == "file" {
        LspClientUrl(specifier.clone())
      } else {
        let specifier_str = if specifier.scheme() == "asset" {
          format!("deno:/asset{}", specifier.path())
        } else if specifier.scheme() == "data" {
          let data_url = deno_graph::source::RawDataUrl::parse(specifier)?;
          let media_type = data_url.media_type();
          let extension = if media_type == MediaType::Unknown {
            ""
          } else {
            media_type.as_ts_extension()
          };
          format!(
            "deno:/{}/data_url{}",
            hash_data_specifier(specifier),
            extension
          )
        } else {
          to_deno_url(specifier)
        };
        let url = LspClientUrl(Url::parse(&specifier_str)?);
        inner.put(specifier.clone(), url.clone());
        url
      };
      Ok(url)
    }
  }

  /// Normalize URLs from the client, where "virtual" `deno:///` URLs are
  /// converted into proper module specifiers, as well as handle situations
  /// where the client encodes a file URL differently than Rust does by default
  /// causing issues with string matching of URLs.
  ///
  /// Note: Sometimes the url provided by the client may not have a trailing slash,
  /// so we need to force it to in the mapping and nee to explicitly state whether
  /// this is a file or directory url.
  pub fn normalize_url(&self, url: &Url, kind: LspUrlKind) -> ModuleSpecifier {
    if let Some(cache) = &self.local_http_cache {
      if url.scheme() == "file" {
        if let Ok(path) = url.to_file_path() {
          if let Some(remote_url) = cache.get_remote_url(&path) {
            return remote_url;
          }
        }
      }
    }
    let mut inner = self.inner.lock();
    if let Some(specifier) = inner.get_specifier(url).cloned() {
      return specifier;
    }
    let mut specifier = None;
    if url.scheme() == "file" {
      if let Ok(path) = url.to_file_path() {
        specifier = Some(match kind {
          LspUrlKind::Folder => Url::from_directory_path(path).unwrap(),
          LspUrlKind::File => Url::from_file_path(path).unwrap(),
        });
      }
    } else if let Some(s) = file_like_to_file_specifier(url) {
      specifier = Some(s);
    } else if let Some(s) = from_deno_url(url) {
      specifier = Some(s);
    }
    let specifier = specifier.unwrap_or_else(|| url.clone());
    inner.put(specifier.clone(), LspClientUrl(url.clone()));
    specifier
  }
}

/// Convert a e.g. `deno-notebook-cell:` specifier to a `file:` specifier.
/// ```rust
/// assert_eq!(
///   file_like_to_file_specifier(
///     &Url::parse("deno-notebook-cell:/path/to/file.ipynb#abc").unwrap(),
///   ),
///   Some(Url::parse("file:///path/to/file.ipynb.ts?scheme=deno-notebook-cell#abc").unwrap()),
/// );
fn file_like_to_file_specifier(specifier: &Url) -> Option<Url> {
  if matches!(specifier.scheme(), "untitled" | "deno-notebook-cell") {
    if let Ok(mut s) = ModuleSpecifier::parse(&format!(
      "file://{}",
      &specifier.as_str()[deno_core::url::quirks::internal_components(specifier)
        .host_end as usize..],
    )) {
      s.query_pairs_mut()
        .append_pair("scheme", specifier.scheme());
      s.set_path(&format!("{}.ts", s.path()));
      return Some(s);
    }
  }
  None
}

