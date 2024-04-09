// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use super::completions::IMPORT_COMMIT_CHARS;
use super::logging::lsp_log;
use super::path_to_regex::parse;
use super::path_to_regex::string_to_regex;
use super::path_to_regex::Compiler;
use super::path_to_regex::Key;
use super::path_to_regex::MatchResult;
use super::path_to_regex::Matcher;
use super::path_to_regex::StringOrNumber;
use super::path_to_regex::StringOrVec;
use super::path_to_regex::Token;

use crate::deno_cli::args::CacheSetting;
use crate::deno_cli::cache::GlobalHttpCache;
use crate::deno_cli::cache::HttpCache;
use crate::deno_cli::file_fetcher::FetchOptions;
use crate::deno_cli::file_fetcher::FileFetcher;
use crate::deno_cli::http_util::HttpClient;

use deno_core::anyhow::anyhow;
use deno_core::error::AnyError;
use deno_core::serde::Deserialize;
use deno_core::serde_json;
use deno_core::serde_json::json;
use deno_core::serde_json::Value;
use deno_core::url::ParseError;
use deno_core::url::Position;
use deno_core::url::Url;
use deno_core::ModuleSpecifier;
use deno_graph::Dependency;
use deno_runtime::permissions::PermissionsContainer;
use log::error;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tower_lsp::lsp_types as lsp;

const CONFIG_PATH: &str = "/.well-known/deno-import-intellisense.json";
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
  .add(b'&')
  .add(b'+')
  .add(b',');

const REGISTRY_IMPORT_COMMIT_CHARS: &[&str] = &["\"", "'"];

static REPLACEMENT_VARIABLE_RE: Lazy<regex::Regex> = lazy_regex::lazy_regex!(r"\$\{\{?(\w+)\}?\}");

fn base_url(url: &Url) -> String {
  url.origin().ascii_serialization()
}

#[derive(Debug)]
enum CompletionType {
  Literal(String),
  Key {
    key: Key,
    prefix: Option<String>,
    index: usize,
  },
}

/// Determine if a completion at a given offset is a string literal or a key/
/// variable.
fn get_completion_type(
  offset: usize,
  tokens: &[Token],
  match_result: &MatchResult,
) -> Option<CompletionType> {
  let mut len = 0_usize;
  for (index, token) in tokens.iter().enumerate() {
    match token {
      Token::String(s) => {
        len += s.chars().count();
        if offset < len {
          return Some(CompletionType::Literal(s.clone()));
        }
      }
      Token::Key(k) => {
        if let Some(prefix) = &k.prefix {
          len += prefix.chars().count();
          if offset < len {
            return Some(CompletionType::Key {
              key: k.clone(),
              prefix: Some(prefix.clone()),
              index,
            });
          }
        }
        if offset < len {
          return None;
        }
        if let StringOrNumber::String(name) = &k.name {
          let value = match_result
            .get(name)
            .map(|s| s.to_string(Some(k), false))
            .unwrap_or_default();
          len += value.chars().count();
          if offset <= len {
            return Some(CompletionType::Key {
              key: k.clone(),
              prefix: None,
              index,
            });
          }
        }
        if let Some(suffix) = &k.suffix {
          len += suffix.chars().count();
          if offset <= len {
            return Some(CompletionType::Literal(suffix.clone()));
          }
        }
      }
    }
  }

  None
}

/// Generate a data value for a completion item that will instruct the client to
/// resolve the completion item to obtain further information, in this case, the
/// details/documentation endpoint for the item if it exists in the registry
/// configuration
fn get_data(
  registry: &RegistryConfiguration,
  base: &ModuleSpecifier,
  variable: &Key,
  value: &str,
) -> Option<Value> {
  let url = registry.get_documentation_url_for_key(variable)?;
  get_endpoint(url, base, variable, Some(value))
    .ok()
    .map(|specifier| json!({ "documentation": specifier }))
}

/// Generate a data value for a completion item that will instruct the client to
/// resolve the completion item to obtain further information, in this case, the
/// details/documentation endpoint for the item if it exists in the registry
/// configuration when there is a match result that should be interpolated
fn get_data_with_match(
  registry: &RegistryConfiguration,
  base: &ModuleSpecifier,
  tokens: &[Token],
  match_result: &MatchResult,
  variable: &Key,
  value: &str,
) -> Option<Value> {
  let url = registry.get_documentation_url_for_key(variable)?;
  get_endpoint_with_match(variable, url, base, tokens, match_result, Some(value))
    .ok()
    .map(|specifier| json!({ "documentation": specifier }))
}

/// Convert a single variable templated string into a fully qualified URL which
/// can be fetched to provide additional data.
fn get_endpoint(
  url: &str,
  base: &Url,
  variable: &Key,
  maybe_value: Option<&str>,
) -> Result<ModuleSpecifier, AnyError> {
  let url = replace_variable(url, variable, maybe_value);
  parse_url_with_base(&url, base)
}

/// Convert a templated URL string into a fully qualified URL which can be
/// fetched to provide additional data. If `maybe_value` is some, then the
/// variable will replaced in the template prior to other matched variables
/// being replaced, otherwise the supplied variable will be blanked out if
/// present in the template.
fn get_endpoint_with_match(
  variable: &Key,
  url: &str,
  base: &Url,
  tokens: &[Token],
  match_result: &MatchResult,
  maybe_value: Option<&str>,
) -> Result<ModuleSpecifier, AnyError> {
  let mut url = url.to_string();
  let has_value = maybe_value.is_some();
  if has_value {
    url = replace_variable(&url, variable, maybe_value);
  }
  for (key, value) in match_result.params.iter() {
    if let StringOrNumber::String(name) = key {
      let maybe_key = tokens.iter().find_map(|t| match t {
        Token::Key(k) if k.name == *key => Some(k),
        _ => None,
      });
      url = url.replace(&format!("${{{name}}}"), &value.to_string(maybe_key, true));
      url = url.replace(
        &format!("${{{{{name}}}}}"),
        &percent_encoding::percent_encode(value.to_string(maybe_key, true).as_bytes(), COMPONENT)
          .to_string(),
      );
    }
  }
  if !has_value {
    url = replace_variable(&url, variable, None);
  }
  parse_url_with_base(&url, base)
}

/// Based on the preselect response from the registry, determine if this item
/// should be preselected or not.
fn get_preselect(
  item: String,
  preselect: Option<String>,
) -> Option<bool> {
  if Some(item) == preselect {
    Some(true)
  } else {
    None
  }
}

fn parse_replacement_variables<S: AsRef<str>>(s: S) -> Vec<String> {
  REPLACEMENT_VARIABLE_RE
    .captures_iter(s.as_ref())
    .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
    .collect()
}

/// Attempt to parse a URL along with a base, where the base will be used if the
/// URL requires one.
fn parse_url_with_base(
  url: &str,
  base: &ModuleSpecifier,
) -> Result<ModuleSpecifier, AnyError> {
  match Url::parse(url) {
    Ok(url) => Ok(url),
    Err(ParseError::RelativeUrlWithoutBase) => base.join(url).map_err(|err| err.into()),
    Err(err) => Err(err.into()),
  }
}

/// Replaces a variable in a templated URL string with the supplied value or
/// "blank" it out if there is no value supplied.
fn replace_variable(
  url: &str,
  variable: &Key,
  maybe_value: Option<&str>,
) -> String {
  let url_str = url.to_string();
  let value = maybe_value.unwrap_or("");
  if let StringOrNumber::String(name) = &variable.name {
    url_str
      .replace(&format!("${{{name}}}"), value)
      .replace(&format! {"${{{{{name}}}}}"}, value)
  } else {
    url_str
  }
}

/// Validate a registry configuration JSON structure.
fn validate_config(config: &RegistryConfigurationJson) -> Result<(), AnyError> {
  if config.version < 1 || config.version > 2 {
    return Err(anyhow!(
      "Invalid registry configuration. Expected version 1 or 2 got {}.",
      config.version
    ));
  }
  for registry in &config.registries {
    let (_, keys) = string_to_regex(&registry.schema, None)?;
    let key_names: Vec<String> = keys
      .map(|keys| {
        keys
          .iter()
          .filter_map(|k| {
            if let StringOrNumber::String(s) = &k.name {
              Some(s.clone())
            } else {
              None
            }
          })
          .collect()
      })
      .unwrap_or_default();

    for key_name in &key_names {
      if !registry
        .variables
        .iter()
        .map(|var| var.key.to_owned())
        .any(|x| x == *key_name)
      {
        return Err(anyhow!("Invalid registry configuration. Registry with schema \"{}\" is missing variable declaration for key \"{}\".", registry.schema, key_name));
      }
    }

    for variable in &registry.variables {
      let key_index = key_names.iter().position(|key| *key == variable.key);
      let key_index = key_index.ok_or_else(||anyhow!("Invalid registry configuration. Registry with schema \"{}\" is missing a path parameter in schema for variable \"{}\".", registry.schema, variable.key))?;

      let replacement_variables = parse_replacement_variables(&variable.url);
      let limited_keys = key_names.get(0..key_index).unwrap();
      for v in replacement_variables {
        if variable.key == v && config.version == 1 {
          return Err(anyhow!("Invalid registry configuration. Url \"{}\" (for variable \"{}\" in registry with schema \"{}\") uses variable \"{}\", which is not allowed because that would be a self reference.", variable.url, variable.key, registry.schema, v));
        }

        let key_index = limited_keys.iter().position(|key| key == &v);

        if key_index.is_none() && variable.key != v {
          return Err(anyhow!("Invalid registry configuration. Url \"{}\" (for variable \"{}\" in registry with schema \"{}\") uses variable \"{}\", which is not allowed because the schema defines \"{}\" to the right of \"{}\".", variable.url, variable.key, registry.schema, v, v, variable.key));
        }
      }
    }
  }

  Ok(())
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistryConfigurationVariable {
  /// The name of the variable.
  key: String,
  /// An optional URL/API endpoint that can provide optional documentation for a
  /// completion item when requested by the language server.
  documentation: Option<String>,
  /// The URL with variable substitutions of the endpoint that will provide
  /// completions for the variable.
  url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistryConfiguration {
  /// A Express-like path which describes how URLs are composed for a registry.
  schema: String,
  /// The variables denoted in the `schema` should have a variable entry.
  variables: Vec<RegistryConfigurationVariable>,
}

impl RegistryConfiguration {
  fn get_url_for_key(
    &self,
    key: &Key,
  ) -> Option<&str> {
    self.variables.iter().find_map(|v| {
      if key.name == StringOrNumber::String(v.key.clone()) {
        Some(v.url.as_str())
      } else {
        None
      }
    })
  }

  fn get_documentation_url_for_key(
    &self,
    key: &Key,
  ) -> Option<&str> {
    self.variables.iter().find_map(|v| {
      if key.name == StringOrNumber::String(v.key.clone()) {
        v.documentation.as_deref()
      } else {
        None
      }
    })
  }
}

/// A structure that represents the configuration of an origin and its module
/// registries.
#[derive(Debug, Deserialize)]
struct RegistryConfigurationJson {
  version: u32,
  registries: Vec<RegistryConfiguration>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VariableItemsList {
  pub items: Vec<String>,
  #[serde(default)]
  pub is_incomplete: bool,
  pub preselect: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum VariableItems {
  Simple(Vec<String>),
  List(VariableItemsList),
}

/// A structure which holds the information about currently configured module
/// registries and can provide completion information for URLs that match
/// one of the enabled registries.
#[derive(Debug, Clone)]
pub struct ModuleRegistry {
  origins: HashMap<String, Vec<RegistryConfiguration>>,
  pub file_fetcher: FileFetcher,
  http_cache: Arc<GlobalHttpCache>,
}

impl ModuleRegistry {
  pub fn new(
    location: PathBuf,
    http_client: Arc<HttpClient>,
  ) -> Self {
    // the http cache should always be the global one for registry completions
    let http_cache = Arc::new(GlobalHttpCache::new(
      location,
      crate::deno_cli::cache::RealDenoCacheEnv,
    ));
    let mut file_fetcher = FileFetcher::new(
      http_cache.clone(),
      CacheSetting::RespectHeaders,
      true,
      http_client,
      Default::default(),
      None,
    );
    file_fetcher.set_download_log_level(super::logging::lsp_log_level());

    Self {
      origins: HashMap::new(),
      file_fetcher,
      http_cache,
    }
  }

  fn complete_literal(
    &self,
    s: String,
    completions: &mut HashMap<String, lsp::CompletionItem>,
    current_specifier: &str,
    offset: usize,
    range: &lsp::Range,
  ) {
    let label = if s.starts_with('/') {
      s[0..].to_string()
    } else {
      s.to_string()
    };
    let full_text = format!(
      "{}{}{}",
      &current_specifier[..offset],
      s,
      &current_specifier[offset..]
    );
    let text_edit = Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
      range: *range,
      new_text: full_text.clone(),
    }));
    let filter_text = Some(full_text);
    completions.insert(
      s,
      lsp::CompletionItem {
        label,
        kind: Some(lsp::CompletionItemKind::FOLDER),
        filter_text,
        sort_text: Some("1".to_string()),
        text_edit,
        commit_characters: Some(
          REGISTRY_IMPORT_COMMIT_CHARS
            .iter()
            .map(|&c| c.into())
            .collect(),
        ),
        ..Default::default()
      },
    );
  }

  /// Disable a registry, removing its configuration, if any, from memory.
  pub async fn disable(
    &mut self,
    origin: &str,
  ) -> Result<(), AnyError> {
    let origin = base_url(&Url::parse(origin)?);
    self.origins.remove(&origin);
    Ok(())
  }

  /// Check to see if the given origin has a registry configuration.
  pub async fn check_origin(
    &self,
    origin: &str,
  ) -> Result<(), AnyError> {
    let origin_url = Url::parse(origin)?;
    let specifier = origin_url.join(CONFIG_PATH)?;
    self.fetch_config(&specifier).await?;
    Ok(())
  }

  /// Fetch and validate the specifier to a registry configuration, resolving
  /// with the configuration if valid.
  async fn fetch_config(
    &self,
    specifier: &ModuleSpecifier,
  ) -> Result<Vec<RegistryConfiguration>, AnyError> {
    let fetch_result = self
      .file_fetcher
      .fetch_with_options(FetchOptions {
        specifier,
        permissions: PermissionsContainer::allow_all(),
        maybe_accept: Some("application/vnd.deno.reg.v2+json, application/vnd.deno.reg.v1+json;q=0.9, application/json;q=0.8"),
        maybe_cache_setting: None,
        maybe_checksum: None,
      })
      .await;
    // if there is an error fetching, we will cache an empty file, so that
    // subsequent requests they are just an empty doc which will error without
    // needing to connect to the remote URL. We will cache it for 1 week.
    if fetch_result.is_err() {
      let mut headers_map = HashMap::new();
      headers_map.insert(
        "cache-control".to_string(),
        "max-age=604800, immutable".to_string(),
      );
      self.http_cache.set(specifier, headers_map, &[])?;
    }
    let file = fetch_result?.into_text_decoded()?;
    let config: RegistryConfigurationJson = serde_json::from_str(&file.source)?;
    validate_config(&config)?;
    Ok(config.registries)
  }

  /// Enable a registry by attempting to retrieve its configuration and
  /// validating it.
  pub async fn enable(
    &mut self,
    origin: &str,
  ) -> Result<(), AnyError> {
    let origin_url = Url::parse(origin)?;
    let origin = base_url(&origin_url);
    #[allow(clippy::map_entry)]
    // we can't use entry().or_insert_with() because we can't use async closures
    if !self.origins.contains_key(&origin) {
      let specifier = origin_url.join(CONFIG_PATH)?;
      match self.fetch_config(&specifier).await {
        Ok(configs) => {
          self.origins.insert(origin, configs);
        }
        Err(err) => {
          lsp_log!(
            "  Error fetching registry config for \"{}\": {}",
            origin,
            err.to_string()
          );
          self.origins.remove(&origin);
        }
      }
    }

    Ok(())
  }

  pub async fn get_hover(
    &self,
    dependency: &Dependency,
  ) -> Option<String> {
    let maybe_code = dependency.get_code();
    let maybe_type = dependency.get_type();
    let specifier = match (maybe_code, maybe_type) {
      (Some(specifier), _) => Some(specifier),
      (_, Some(specifier)) => Some(specifier),
      _ => None,
    }?;
    let origin = base_url(specifier);
    let registries = self.origins.get(&origin)?;
    let path = &specifier[Position::BeforePath..];
    for registry in registries {
      let tokens = parse(&registry.schema, None).ok()?;
      let matcher = Matcher::new(&tokens, None).ok()?;
      if let Some(match_result) = matcher.matches(path) {
        let key = if let Some(Token::Key(key)) = tokens.iter().last() {
          Some(key)
        } else {
          None
        }?;
        let url = registry.get_documentation_url_for_key(key)?;
        let endpoint =
          get_endpoint_with_match(key, url, specifier, &tokens, &match_result, None).ok()?;
        let file = self
          .file_fetcher
          .fetch(&endpoint, PermissionsContainer::allow_all())
          .await
          .ok()?
          .into_text_decoded()
          .ok()?;
        let documentation: lsp::Documentation = serde_json::from_str(&file.source).ok()?;
        return match documentation {
          lsp::Documentation::String(doc) => Some(doc),
          lsp::Documentation::MarkupContent(lsp::MarkupContent { value, .. }) => Some(value),
        };
      }
    }

    None
  }

  /// For a string specifier from the client, provide a set of completions, if
  /// any, for the specifier.
  pub async fn get_completions(
    &self,
    current_specifier: &str,
    offset: usize,
    range: &lsp::Range,
    specifier_exists: impl Fn(&ModuleSpecifier) -> bool,
  ) -> Option<lsp::CompletionList> {
    if let Ok(specifier) = Url::parse(current_specifier) {
      let origin = base_url(&specifier);
      let origin_len = origin.chars().count();
      if offset >= origin_len {
        if let Some(registries) = self.origins.get(&origin) {
          let path = &specifier[Position::BeforePath..];
          let path_offset = offset - origin_len;
          let mut completions = HashMap::<String, lsp::CompletionItem>::new();
          let mut is_incomplete = false;
          let mut did_match = false;
          for registry in registries {
            let tokens = parse(&registry.schema, None)
              .map_err(|e| {
                error!(
                  "Error parsing registry schema for origin \"{}\". {}",
                  origin, e
                );
              })
              .ok()?;
            let mut i = tokens.len();
            let last_key_name = StringOrNumber::String(
              tokens
                .iter()
                .last()
                .map(|t| {
                  if let Token::Key(key) = t {
                    if let StringOrNumber::String(s) = &key.name {
                      return s.clone();
                    }
                  }
                  "".to_string()
                })
                .unwrap_or_default(),
            );
            loop {
              let matcher = Matcher::new(&tokens[..i], None)
                .map_err(|e| {
                  error!(
                    "Error creating matcher for schema for origin \"{}\". {}",
                    origin, e
                  );
                })
                .ok()?;
              if let Some(match_result) = matcher.matches(path) {
                did_match = true;
                let completion_type = get_completion_type(path_offset, &tokens, &match_result);
                match completion_type {
                  Some(CompletionType::Literal(s)) => {
                    self.complete_literal(s, &mut completions, current_specifier, offset, range)
                  }
                  Some(CompletionType::Key { key, prefix, index }) => {
                    let maybe_url = registry.get_url_for_key(&key);
                    if let Some(url) = maybe_url {
                      if let Some(items) = self
                        .get_variable_items(&key, url, &specifier, &tokens, &match_result)
                        .await
                      {
                        let compiler = Compiler::new(&tokens[..=index], None);
                        let base = Url::parse(&origin).ok()?;
                        let (items, preselect, incomplete) = match items {
                          VariableItems::List(list) => {
                            (list.items, list.preselect, list.is_incomplete)
                          }
                          VariableItems::Simple(items) => (items, None, false),
                        };
                        if incomplete {
                          is_incomplete = true;
                        }
                        for (idx, item) in items.into_iter().enumerate() {
                          let mut label = if let Some(p) = &prefix {
                            format!("{p}{item}")
                          } else {
                            item.clone()
                          };
                          if label.ends_with('/') {
                            label.pop();
                          }
                          let kind = if key.name == last_key_name && !item.ends_with('/') {
                            Some(lsp::CompletionItemKind::FILE)
                          } else {
                            Some(lsp::CompletionItemKind::FOLDER)
                          };
                          let mut params = match_result.params.clone();
                          params.insert(key.name.clone(), StringOrVec::from_str(&item, &key));
                          let mut path = compiler.to_path(&params).unwrap_or_default();
                          if path.ends_with('/') {
                            path.pop();
                          }
                          let item_specifier = base.join(&path).ok()?;
                          let full_text = item_specifier.as_str();
                          let text_edit = Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: *range,
                            new_text: full_text.to_string(),
                          }));
                          let command = if key.name == last_key_name
                            && !item.ends_with('/')
                            && !specifier_exists(&item_specifier)
                          {
                            Some(lsp::Command {
                              title: "".to_string(),
                              command: "deno.cache".to_string(),
                              arguments: Some(vec![json!([item_specifier]), json!(&specifier)]),
                            })
                          } else {
                            None
                          };
                          let detail = Some(format!("({})", key.name));
                          let filter_text = Some(full_text.to_string());
                          let sort_text = Some(format!("{:0>10}", idx + 1));
                          let preselect = get_preselect(item.clone(), preselect.clone());
                          let data = get_data_with_match(
                            registry,
                            &specifier,
                            &tokens,
                            &match_result,
                            &key,
                            &item,
                          );
                          let commit_characters = if is_incomplete {
                            Some(
                              REGISTRY_IMPORT_COMMIT_CHARS
                                .iter()
                                .map(|&c| c.into())
                                .collect(),
                            )
                          } else {
                            Some(IMPORT_COMMIT_CHARS.iter().map(|&c| c.into()).collect())
                          };
                          completions.insert(
                            item,
                            lsp::CompletionItem {
                              label,
                              kind,
                              detail,
                              sort_text,
                              filter_text,
                              text_edit,
                              command,
                              preselect,
                              data,
                              commit_characters,
                              ..Default::default()
                            },
                          );
                        }
                      }
                    }
                  }
                  None => (),
                }
                break;
              }
              i -= 1;
              // If we have fallen though to the first token, and we still
              // didn't get a match
              if i == 0 {
                match &tokens[i] {
                  // so if the first token is a string literal, we will return
                  // that as a suggestion
                  Token::String(s) => {
                    if s.starts_with(path) {
                      let label = s.to_string();
                      let kind = Some(lsp::CompletionItemKind::FOLDER);
                      let mut url = specifier.clone();
                      url.set_path(s);
                      let full_text = url.as_str();
                      let text_edit = Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                        range: *range,
                        new_text: full_text.to_string(),
                      }));
                      let filter_text = Some(full_text.to_string());
                      completions.insert(
                        s.to_string(),
                        lsp::CompletionItem {
                          label,
                          kind,
                          filter_text,
                          sort_text: Some("1".to_string()),
                          text_edit,
                          preselect: Some(true),
                          commit_characters: Some(
                            REGISTRY_IMPORT_COMMIT_CHARS
                              .iter()
                              .map(|&c| c.into())
                              .collect(),
                          ),
                          ..Default::default()
                        },
                      );
                    }
                  }
                  // if the token though is a key, and the key has a prefix, and
                  // the path matches the prefix, we will go and get the items
                  // for that first key and return them.
                  Token::Key(k) => {
                    if let Some(prefix) = &k.prefix {
                      let maybe_url = registry.get_url_for_key(k);
                      if let Some(url) = maybe_url {
                        if let Some(items) = self.get_items(url).await {
                          let base = Url::parse(&origin).ok()?;
                          let (items, preselect, incomplete) = match items {
                            VariableItems::List(list) => {
                              (list.items, list.preselect, list.is_incomplete)
                            }
                            VariableItems::Simple(items) => (items, None, false),
                          };
                          if incomplete {
                            is_incomplete = true;
                          }
                          for (idx, item) in items.into_iter().enumerate() {
                            let path = format!("{prefix}{item}");
                            let kind = Some(lsp::CompletionItemKind::FOLDER);
                            let item_specifier = base.join(&path).ok()?;
                            let full_text = item_specifier.as_str();
                            let text_edit = Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                              range: *range,
                              new_text: full_text.to_string(),
                            }));
                            let command =
                              if k.name == last_key_name && !specifier_exists(&item_specifier) {
                                Some(lsp::Command {
                                  title: "".to_string(),
                                  command: "deno.cache".to_string(),
                                  arguments: Some(vec![json!([item_specifier]), json!(&specifier)]),
                                })
                              } else {
                                None
                              };
                            let detail = Some(format!("({})", k.name));
                            let filter_text = Some(full_text.to_string());
                            let sort_text = Some(format!("{:0>10}", idx + 1));
                            let preselect = get_preselect(item.clone(), preselect.clone());
                            let data = get_data(registry, &specifier, k, &path);
                            let commit_characters = if is_incomplete {
                              Some(
                                REGISTRY_IMPORT_COMMIT_CHARS
                                  .iter()
                                  .map(|&c| c.into())
                                  .collect(),
                              )
                            } else {
                              Some(IMPORT_COMMIT_CHARS.iter().map(|&c| c.into()).collect())
                            };
                            completions.insert(
                              item.clone(),
                              lsp::CompletionItem {
                                label: item,
                                kind,
                                detail,
                                sort_text,
                                filter_text,
                                text_edit,
                                command,
                                preselect,
                                data,
                                commit_characters,
                                ..Default::default()
                              },
                            );
                          }
                        }
                      }
                    }
                  }
                }
                break;
              }
            }
          }
          // If we return None, other sources of completions will be looked for
          // but if we did at least match part of a registry, we should send an
          // empty vector so that no-completions will be sent back to the client
          return if completions.is_empty() && !did_match {
            None
          } else {
            Some(lsp::CompletionList {
              items: completions.into_values().collect(),
              is_incomplete,
            })
          };
        }
      }
    }

    self.get_origin_completions(current_specifier, range)
  }

  pub async fn get_documentation(
    &self,
    url: &str,
  ) -> Option<lsp::Documentation> {
    let specifier = Url::parse(url).ok()?;
    let file = self
      .file_fetcher
      .fetch(&specifier, PermissionsContainer::allow_all())
      .await
      .ok()?
      .into_text_decoded()
      .ok()?;
    serde_json::from_str(&file.source).ok()
  }

  pub fn get_origin_completions(
    &self,
    current_specifier: &str,
    range: &lsp::Range,
  ) -> Option<lsp::CompletionList> {
    let items = self
      .origins
      .keys()
      .filter_map(|k| {
        let mut origin = k.to_string();
        if origin.ends_with('/') {
          origin.pop();
        }
        if origin.starts_with(current_specifier) {
          let text_edit = Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
            range: *range,
            new_text: origin.clone(),
          }));
          Some(lsp::CompletionItem {
            label: origin,
            kind: Some(lsp::CompletionItemKind::FOLDER),
            detail: Some("(registry)".to_string()),
            sort_text: Some("2".to_string()),
            text_edit,
            commit_characters: Some(
              REGISTRY_IMPORT_COMMIT_CHARS
                .iter()
                .map(|&c| c.into())
                .collect(),
            ),
            ..Default::default()
          })
        } else {
          None
        }
      })
      .collect::<Vec<lsp::CompletionItem>>();
    if !items.is_empty() {
      Some(lsp::CompletionList {
        items,
        is_incomplete: false,
      })
    } else {
      None
    }
  }

  async fn get_items(
    &self,
    url: &str,
  ) -> Option<VariableItems> {
    let specifier = ModuleSpecifier::parse(url).ok()?;
    let file = self
      .file_fetcher
      .fetch(&specifier, PermissionsContainer::allow_all())
      .await
      .map_err(|err| {
        error!(
          "Internal error fetching endpoint \"{}\". {}",
          specifier, err
        );
      })
      .ok()?
      .into_text_decoded()
      .ok()?;
    let items: VariableItems = serde_json::from_str(&file.source)
      .map_err(|err| {
        error!(
          "Error parsing response from endpoint \"{}\". {}",
          specifier, err
        );
      })
      .ok()?;
    Some(items)
  }

  async fn get_variable_items(
    &self,
    variable: &Key,
    url: &str,
    base: &Url,
    tokens: &[Token],
    match_result: &MatchResult,
  ) -> Option<VariableItems> {
    let specifier = get_endpoint_with_match(variable, url, base, tokens, match_result, None)
      .map_err(|err| {
        error!("Internal error mapping endpoint \"{}\". {}", url, err);
      })
      .ok()?;
    let file = self
      .file_fetcher
      .fetch(&specifier, PermissionsContainer::allow_all())
      .await
      .map_err(|err| {
        error!(
          "Internal error fetching endpoint \"{}\". {}",
          specifier, err
        );
      })
      .ok()?
      .into_text_decoded()
      .ok()?;
    let items: VariableItems = serde_json::from_str(&file.source)
      .map_err(|err| {
        error!(
          "Error parsing response from endpoint \"{}\". {}",
          specifier, err
        );
      })
      .ok()?;
    Some(items)
  }
}
