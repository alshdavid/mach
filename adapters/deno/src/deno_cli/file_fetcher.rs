// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use crate::deno_cli::args::CacheSetting;
use crate::deno_cli::auth_tokens::AuthToken;
use crate::deno_cli::auth_tokens::AuthTokens;
use crate::deno_cli::cache::HttpCache;
use crate::deno_cli::colors;
use crate::deno_cli::http_util;
use crate::deno_cli::http_util::resolve_redirect_from_response;
use crate::deno_cli::http_util::CacheSemantics;
use crate::deno_cli::http_util::HeadersMap;
use crate::deno_cli::http_util::HttpClient;
use crate::deno_cli::util::progress_bar::ProgressBar;
use crate::deno_cli::util::progress_bar::UpdateGuard;

use deno_ast::MediaType;
use deno_core::anyhow::Context;
use deno_core::error::custom_error;
use deno_core::error::generic_error;
use deno_core::error::uri_error;
use deno_core::error::AnyError;
use deno_core::futures;
use deno_core::futures::future::FutureExt;
use deno_core::parking_lot::Mutex;
use deno_core::url::Url;
use deno_core::ModuleSpecifier;
use deno_graph::source::LoaderChecksum;
use deno_runtime::deno_fetch::reqwest::header::HeaderValue;
use deno_runtime::deno_fetch::reqwest::header::ACCEPT;
use deno_runtime::deno_fetch::reqwest::header::AUTHORIZATION;
use deno_runtime::deno_fetch::reqwest::header::IF_NONE_MATCH;
use deno_runtime::deno_fetch::reqwest::StatusCode;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::PermissionsContainer;
use log::debug;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;

pub const SUPPORTED_SCHEMES: [&str; 5] =
  ["data", "blob", "file", "http", "https"];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextDecodedFile {
  pub media_type: MediaType,
  /// The _final_ specifier for the file.  The requested specifier and the final
  /// specifier maybe different for remote files that have been redirected.
  pub specifier: ModuleSpecifier,
  /// The source of the file.
  pub source: Arc<str>,
}

/// A structure representing a source file.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct File {
  /// The _final_ specifier for the file.  The requested specifier and the final
  /// specifier maybe different for remote files that have been redirected.
  pub specifier: ModuleSpecifier,
  pub maybe_headers: Option<HashMap<String, String>>,
  /// The source of the file.
  pub source: Arc<[u8]>,
}

impl File {
  pub fn resolve_media_type_and_charset(&self) -> (MediaType, Option<&str>) {
    deno_graph::source::resolve_media_type_and_charset_from_headers(
      &self.specifier,
      self.maybe_headers.as_ref(),
    )
  }

  /// Decodes the source bytes into a string handling any encoding rules
  /// for local vs remote files and dealing with the charset.
  pub fn into_text_decoded(self) -> Result<TextDecodedFile, AnyError> {
    // lots of borrow checker fighting here
    let (media_type, maybe_charset) =
      deno_graph::source::resolve_media_type_and_charset_from_headers(
        &self.specifier,
        self.maybe_headers.as_ref(),
      );
    let specifier = self.specifier;
    match deno_graph::source::decode_source(
      &specifier,
      self.source,
      maybe_charset,
    ) {
      Ok(source) => Ok(TextDecodedFile {
        media_type,
        specifier,
        source,
      }),
      Err(err) => {
        Err(err).with_context(|| format!("Failed decoding \"{}\".", specifier))
      }
    }
  }
}

#[derive(Debug, Clone, Default)]
struct MemoryFiles(Arc<Mutex<HashMap<ModuleSpecifier, File>>>);

impl MemoryFiles {
  pub fn get(&self, specifier: &ModuleSpecifier) -> Option<File> {
    self.0.lock().get(specifier).cloned()
  }

  pub fn insert(&self, specifier: ModuleSpecifier, file: File) -> Option<File> {
    self.0.lock().insert(specifier, file)
  }
}

/// Fetch a source file from the local file system.
fn fetch_local(specifier: &ModuleSpecifier) -> Result<File, AnyError> {
  let local = specifier.to_file_path().map_err(|_| {
    uri_error(format!("Invalid file path.\n  Specifier: {specifier}"))
  })?;
  let bytes = fs::read(local)?;

  Ok(File {
    specifier: specifier.clone(),
    maybe_headers: None,
    source: bytes.into(),
  })
}

/// Return a validated scheme for a given module specifier.
fn get_validated_scheme(
  specifier: &ModuleSpecifier,
) -> Result<String, AnyError> {
  let scheme = specifier.scheme();
  if !SUPPORTED_SCHEMES.contains(&scheme) {
    Err(generic_error(format!(
      "Unsupported scheme \"{scheme}\" for module \"{specifier}\". Supported schemes: {SUPPORTED_SCHEMES:#?}"
    )))
  } else {
    Ok(scheme.to_string())
  }
}

pub struct FetchOptions<'a> {
  pub specifier: &'a ModuleSpecifier,
  pub permissions: PermissionsContainer,
  pub maybe_accept: Option<&'a str>,
  pub maybe_cache_setting: Option<&'a CacheSetting>,
  pub maybe_checksum: Option<LoaderChecksum>,
}

/// A structure for resolving, fetching and caching source files.
#[derive(Debug, Clone)]
pub struct FileFetcher {
  auth_tokens: AuthTokens,
  allow_remote: bool,
  memory_files: MemoryFiles,
  cache_setting: CacheSetting,
  http_cache: Arc<dyn HttpCache>,
  http_client: Arc<HttpClient>,
  blob_store: Arc<BlobStore>,
  download_log_level: log::Level,
  progress_bar: Option<ProgressBar>,
}

impl FileFetcher {
  pub fn new(
    http_cache: Arc<dyn HttpCache>,
    cache_setting: CacheSetting,
    allow_remote: bool,
    http_client: Arc<HttpClient>,
    blob_store: Arc<BlobStore>,
    progress_bar: Option<ProgressBar>,
  ) -> Self {
    Self {
      auth_tokens: AuthTokens::new(env::var("DENO_AUTH_TOKENS").ok()),
      allow_remote,
      memory_files: Default::default(),
      cache_setting,
      http_cache,
      http_client,
      blob_store,
      download_log_level: log::Level::Info,
      progress_bar,
    }
  }

  pub fn cache_setting(&self) -> &CacheSetting {
    &self.cache_setting
  }

  /// Sets the log level to use when outputting the download message.
  pub fn set_download_log_level(&mut self, level: log::Level) {
    self.download_log_level = level;
  }

  /// Fetch cached remote file.
  ///
  /// This is a recursive operation if source file has redirections.
  pub fn fetch_cached(
    &self,
    specifier: &ModuleSpecifier,
    maybe_checksum: Option<LoaderChecksum>,
    redirect_limit: i64,
  ) -> Result<Option<File>, AnyError> {
    debug!("FileFetcher::fetch_cached - specifier: {}", specifier);
    if redirect_limit < 0 {
      return Err(custom_error("Http", "Too many redirects."));
    }

    let cache_key = self.http_cache.cache_item_key(specifier)?; // compute this once
    let Some(headers) = self.http_cache.read_headers(&cache_key)? else {
      return Ok(None);
    };
    if let Some(redirect_to) = headers.get("location") {
      let redirect =
        deno_core::resolve_import(redirect_to, specifier.as_str())?;
      return self.fetch_cached(&redirect, maybe_checksum, redirect_limit - 1);
    }
    let Some(bytes) = self.http_cache.read_file_bytes(
      &cache_key,
      maybe_checksum
        .as_ref()
        .map(|c| deno_cache_dir::Checksum::new(c.as_str())),
      deno_cache_dir::GlobalToLocalCopy::Allow,
    )?
    else {
      return Ok(None);
    };

    Ok(Some(File {
      specifier: specifier.clone(),
      maybe_headers: Some(headers),
      source: Arc::from(bytes),
    }))
  }

  /// Convert a data URL into a file, resulting in an error if the URL is
  /// invalid.
  fn fetch_data_url(
    &self,
    specifier: &ModuleSpecifier,
  ) -> Result<File, AnyError> {
    debug!("FileFetcher::fetch_data_url() - specifier: {}", specifier);
    let data_url = deno_graph::source::RawDataUrl::parse(specifier)?;
    let (bytes, headers) = data_url.into_bytes_and_headers();
    Ok(File {
      specifier: specifier.clone(),
      maybe_headers: Some(headers),
      source: Arc::from(bytes),
    })
  }

  /// Get a blob URL.
  async fn fetch_blob_url(
    &self,
    specifier: &ModuleSpecifier,
  ) -> Result<File, AnyError> {
    debug!("FileFetcher::fetch_blob_url() - specifier: {}", specifier);
    let blob = self
      .blob_store
      .get_object_url(specifier.clone())
      .ok_or_else(|| {
        custom_error(
          "NotFound",
          format!("Blob URL not found: \"{specifier}\"."),
        )
      })?;

    let bytes = blob.read_all().await?;
    let headers =
      HashMap::from([("content-type".to_string(), blob.media_type.clone())]);

    Ok(File {
      specifier: specifier.clone(),
      maybe_headers: Some(headers),
      source: Arc::from(bytes),
    })
  }

  /// Asynchronously fetch remote source file specified by the URL following
  /// redirects.
  ///
  /// **Note** this is a recursive method so it can't be "async", but needs to
  /// return a `Pin<Box<..>>`.
  fn fetch_remote(
    &self,
    specifier: &ModuleSpecifier,
    permissions: PermissionsContainer,
    redirect_limit: i64,
    maybe_accept: Option<String>,
    cache_setting: &CacheSetting,
    maybe_checksum: Option<LoaderChecksum>,
  ) -> Pin<Box<dyn Future<Output = Result<File, AnyError>> + Send>> {
    debug!("FileFetcher::fetch_remote() - specifier: {}", specifier);
    if redirect_limit < 0 {
      return futures::future::err(custom_error("Http", "Too many redirects."))
        .boxed();
    }

    if let Err(err) = permissions.check_specifier(specifier) {
      return futures::future::err(err).boxed();
    }

    if self.should_use_cache(specifier, cache_setting) {
      match self.fetch_cached(specifier, maybe_checksum.clone(), redirect_limit)
      {
        Ok(Some(file)) => {
          return futures::future::ok(file).boxed();
        }
        Ok(None) => {}
        Err(err) => {
          return futures::future::err(err).boxed();
        }
      }
    }

    if *cache_setting == CacheSetting::Only {
      return futures::future::err(custom_error(
        "NotCached",
        format!(
          "Specifier not found in cache: \"{specifier}\", --cached-only is specified."
        ),
      ))
      .boxed();
    }

    let mut maybe_progress_guard = None;
    if let Some(pb) = self.progress_bar.as_ref() {
      maybe_progress_guard = Some(pb.update(specifier.as_str()));
    } else {
      log::log!(
        self.download_log_level,
        "{} {}",
        colors::green("Download"),
        specifier
      );
    }

    let maybe_etag = self
      .http_cache
      .cache_item_key(specifier)
      .ok()
      .and_then(|key| self.http_cache.read_headers(&key).ok().flatten())
      .and_then(|headers| headers.get("etag").cloned());
    let maybe_auth_token = self.auth_tokens.get(specifier);
    let specifier = specifier.clone();
    let client = self.http_client.clone();
    let file_fetcher = self.clone();
    let cache_setting = cache_setting.clone();
    // A single pass of fetch either yields code or yields a redirect, server
    // error causes a single retry to avoid crashing hard on intermittent failures.

    async fn handle_request_or_server_error(
      retried: &mut bool,
      specifier: &Url,
      err_str: String,
    ) -> Result<(), AnyError> {
      // Retry once, and bail otherwise.
      if !*retried {
        *retried = true;
        log::debug!("Import '{}' failed: {}. Retrying...", specifier, err_str);
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        Ok(())
      } else {
        Err(generic_error(format!(
          "Import '{}' failed: {}",
          specifier, err_str
        )))
      }
    }

    async move {
      let mut retried = false;
      let result = loop {
        let result = match fetch_once(
          &client,
          FetchOnceArgs {
            url: specifier.clone(),
            maybe_accept: maybe_accept.clone(),
            maybe_etag: maybe_etag.clone(),
            maybe_auth_token: maybe_auth_token.clone(),
            maybe_progress_guard: maybe_progress_guard.as_ref(),
          },
        )
        .await?
        {
          FetchOnceResult::NotModified => {
            let file = file_fetcher
              .fetch_cached(&specifier, maybe_checksum, 10)?
              .unwrap();
            Ok(file)
          }
          FetchOnceResult::Redirect(redirect_url, headers) => {
            file_fetcher.http_cache.set(&specifier, headers, &[])?;
            file_fetcher
              .fetch_remote(
                &redirect_url,
                permissions,
                redirect_limit - 1,
                maybe_accept,
                &cache_setting,
                maybe_checksum,
              )
              .await
          }
          FetchOnceResult::Code(bytes, headers) => {
            file_fetcher
              .http_cache
              .set(&specifier, headers.clone(), &bytes)?;
            if let Some(checksum) = &maybe_checksum {
              checksum.check_source(&bytes)?;
            }
            Ok(File {
              specifier,
              maybe_headers: Some(headers),
              source: Arc::from(bytes),
            })
          }
          FetchOnceResult::RequestError(err) => {
            handle_request_or_server_error(&mut retried, &specifier, err)
              .await?;
            continue;
          }
          FetchOnceResult::ServerError(status) => {
            handle_request_or_server_error(
              &mut retried,
              &specifier,
              status.to_string(),
            )
            .await?;
            continue;
          }
        };
        break result;
      };

      drop(maybe_progress_guard);
      result
    }
    .boxed()
  }

  /// Returns if the cache should be used for a given specifier.
  fn should_use_cache(
    &self,
    specifier: &ModuleSpecifier,
    cache_setting: &CacheSetting,
  ) -> bool {
    match cache_setting {
      CacheSetting::ReloadAll => false,
      CacheSetting::Use | CacheSetting::Only => true,
      CacheSetting::RespectHeaders => {
        let Ok(cache_key) = self.http_cache.cache_item_key(specifier) else {
          return false;
        };
        let Ok(Some(headers)) = self.http_cache.read_headers(&cache_key) else {
          return false;
        };
        let Ok(Some(download_time)) =
          self.http_cache.read_download_time(&cache_key)
        else {
          return false;
        };
        let cache_semantics =
          CacheSemantics::new(headers, download_time, SystemTime::now());
        cache_semantics.should_use()
      }
      CacheSetting::ReloadSome(list) => {
        let mut url = specifier.clone();
        url.set_fragment(None);
        if list.iter().any(|x| x == url.as_str()) {
          return false;
        }
        url.set_query(None);
        let mut path = PathBuf::from(url.as_str());
        loop {
          if list.contains(&path.to_str().unwrap().to_string()) {
            return false;
          }
          if !path.pop() {
            break;
          }
        }
        true
      }
    }
  }

  /// Fetch a source file and asynchronously return it.
  pub async fn fetch(
    &self,
    specifier: &ModuleSpecifier,
    permissions: PermissionsContainer,
  ) -> Result<File, AnyError> {
    self
      .fetch_with_options(FetchOptions {
        specifier,
        permissions,
        maybe_accept: None,
        maybe_cache_setting: None,
        maybe_checksum: None,
      })
      .await
  }

  pub async fn fetch_with_options(
    &self,
    options: FetchOptions<'_>,
  ) -> Result<File, AnyError> {
    let specifier = options.specifier;
    debug!("FileFetcher::fetch() - specifier: {}", specifier);
    let scheme = get_validated_scheme(specifier)?;
    options.permissions.check_specifier(specifier)?;
    if let Some(file) = self.memory_files.get(specifier) {
      Ok(file)
    } else if scheme == "file" {
      // we do not in memory cache files, as this would prevent files on the
      // disk changing effecting things like workers and dynamic imports.
      fetch_local(specifier)
    } else if scheme == "data" {
      self.fetch_data_url(specifier)
    } else if scheme == "blob" {
      self.fetch_blob_url(specifier).await
    } else if !self.allow_remote {
      Err(custom_error(
        "NoRemote",
        format!("A remote specifier was requested: \"{specifier}\", but --no-remote is specified."),
      ))
    } else {
      self
        .fetch_remote(
          specifier,
          options.permissions,
          10,
          options.maybe_accept.map(String::from),
          options.maybe_cache_setting.unwrap_or(&self.cache_setting),
          options.maybe_checksum,
        )
        .await
    }
  }

  /// A synchronous way to retrieve a source file, where if the file has already
  /// been cached in memory it will be returned, otherwise for local files will
  /// be read from disk.
  pub fn get_source(&self, specifier: &ModuleSpecifier) -> Option<File> {
    let maybe_file = self.memory_files.get(specifier);
    if maybe_file.is_none() {
      let is_local = specifier.scheme() == "file";
      if is_local {
        if let Ok(file) = fetch_local(specifier) {
          return Some(file);
        }
      }
      None
    } else {
      maybe_file
    }
  }

  /// Insert a temporary module for the file fetcher.
  pub fn insert_memory_files(&self, file: File) -> Option<File> {
    self.memory_files.insert(file.specifier.clone(), file)
  }
}

#[derive(Debug, Eq, PartialEq)]
enum FetchOnceResult {
  Code(Vec<u8>, HeadersMap),
  NotModified,
  Redirect(Url, HeadersMap),
  RequestError(String),
  ServerError(StatusCode),
}

#[derive(Debug)]
struct FetchOnceArgs<'a> {
  pub url: Url,
  pub maybe_accept: Option<String>,
  pub maybe_etag: Option<String>,
  pub maybe_auth_token: Option<AuthToken>,
  pub maybe_progress_guard: Option<&'a UpdateGuard>,
}

/// Asynchronously fetches the given HTTP URL one pass only.
/// If no redirect is present and no error occurs,
/// yields Code(ResultPayload).
/// If redirect occurs, does not follow and
/// yields Redirect(url).
async fn fetch_once<'a>(
  http_client: &HttpClient,
  args: FetchOnceArgs<'a>,
) -> Result<FetchOnceResult, AnyError> {
  let mut request = http_client.get_no_redirect(args.url.clone())?;

  if let Some(etag) = args.maybe_etag {
    let if_none_match_val = HeaderValue::from_str(&etag)?;
    request = request.header(IF_NONE_MATCH, if_none_match_val);
  }
  if let Some(auth_token) = args.maybe_auth_token {
    let authorization_val = HeaderValue::from_str(&auth_token.to_string())?;
    request = request.header(AUTHORIZATION, authorization_val);
  }
  if let Some(accept) = args.maybe_accept {
    let accepts_val = HeaderValue::from_str(&accept)?;
    request = request.header(ACCEPT, accepts_val);
  }
  let response = match request.send().await {
    Ok(resp) => resp,
    Err(err) => {
      if err.is_connect() || err.is_timeout() {
        return Ok(FetchOnceResult::RequestError(err.to_string()));
      }
      return Err(err.into());
    }
  };

  if response.status() == StatusCode::NOT_MODIFIED {
    return Ok(FetchOnceResult::NotModified);
  }

  let mut result_headers = HashMap::new();
  let response_headers = response.headers();

  if let Some(warning) = response_headers.get("X-Deno-Warning") {
    log::warn!(
      "{} {}",
      crate::deno_cli::colors::yellow("Warning"),
      warning.to_str().unwrap()
    );
  }

  for key in response_headers.keys() {
    let key_str = key.to_string();
    let values = response_headers.get_all(key);
    let values_str = values
      .iter()
      .map(|e| e.to_str().unwrap().to_string())
      .collect::<Vec<String>>()
      .join(",");
    result_headers.insert(key_str, values_str);
  }

  if response.status().is_redirection() {
    let new_url = resolve_redirect_from_response(&args.url, &response)?;
    return Ok(FetchOnceResult::Redirect(new_url, result_headers));
  }

  let status = response.status();

  if status.is_server_error() {
    return Ok(FetchOnceResult::ServerError(status));
  }

  if status.is_client_error() {
    let err = if response.status() == StatusCode::NOT_FOUND {
      custom_error(
        "NotFound",
        format!("Import '{}' failed, not found.", args.url),
      )
    } else {
      generic_error(format!(
        "Import '{}' failed: {}",
        args.url,
        response.status()
      ))
    };
    return Err(err);
  }

  let body = http_util::get_response_body_with_progress(
    response,
    args.maybe_progress_guard,
  )
  .await?;

  Ok(FetchOnceResult::Code(body, result_headers))
}
