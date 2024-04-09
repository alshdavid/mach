// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use super::logging::lsp_log;
use crate::deno_cli::args::ConfigFile;
use crate::deno_cli::lsp::logging::lsp_warn;
use crate::deno_cli::util::fs::canonicalize_path_maybe_not_exists;
use crate::deno_cli::util::path::specifier_to_file_path;
use deno_ast::MediaType;
use deno_config::glob::PathOrPattern;
use deno_config::glob::PathOrPatternSet;
use deno_config::FmtOptionsConfig;
use deno_core::parking_lot::Mutex;
use deno_core::serde::de::DeserializeOwned;
use deno_core::serde::Deserialize;
use deno_core::serde::Serialize;
use deno_core::serde_json;
use deno_core::serde_json::Value;
use deno_core::ModuleSpecifier;
use deno_lockfile::Lockfile;
use lsp::Url;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tower_lsp::lsp_types as lsp;

pub const SETTINGS_SECTION: &str = "deno";

#[derive(Debug, Clone, Default)]
pub struct ClientCapabilities {
  pub code_action_disabled_support: bool,
  pub line_folding_only: bool,
  pub snippet_support: bool,
  pub status_notification: bool,
  /// The client provides the `experimental.testingApi` capability, which is
  /// built around VSCode's testing API. It indicates that the server should
  /// send notifications about tests discovered in modules.
  pub testing_api: bool,
  pub workspace_configuration: bool,
  pub workspace_did_change_watched_files: bool,
  pub workspace_will_rename_files: bool,
}

fn is_true() -> bool {
  true
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensSettings {
  /// Flag for providing implementation code lenses.
  #[serde(default)]
  pub implementations: bool,
  /// Flag for providing reference code lenses.
  #[serde(default)]
  pub references: bool,
  /// Flag for providing reference code lens on all functions.  For this to have
  /// an impact, the `references` flag needs to be `true`.
  #[serde(default)]
  pub references_all_functions: bool,
  /// Flag for providing test code lens on `Deno.test` statements.  There is
  /// also the `test_args` setting, but this is not used by the server.
  #[serde(default = "is_true")]
  pub test: bool,
}

impl Default for CodeLensSettings {
  fn default() -> Self {
    Self {
      implementations: false,
      references: false,
      references_all_functions: false,
      test: true,
    }
  }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DenoCompletionSettings {
  #[serde(default)]
  pub imports: ImportCompletionSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClassMemberSnippets {
  #[serde(default = "is_true")]
  pub enabled: bool,
}

impl Default for ClassMemberSnippets {
  fn default() -> Self {
    Self { enabled: true }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ObjectLiteralMethodSnippets {
  #[serde(default = "is_true")]
  pub enabled: bool,
}

impl Default for ObjectLiteralMethodSnippets {
  fn default() -> Self {
    Self { enabled: true }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompletionSettings {
  #[serde(default)]
  pub complete_function_calls: bool,
  #[serde(default = "is_true")]
  pub include_automatic_optional_chain_completions: bool,
  #[serde(default = "is_true")]
  pub include_completions_for_import_statements: bool,
  #[serde(default = "is_true")]
  pub names: bool,
  #[serde(default = "is_true")]
  pub paths: bool,
  #[serde(default = "is_true")]
  pub auto_imports: bool,
  #[serde(default = "is_true")]
  pub enabled: bool,
  #[serde(default)]
  pub class_member_snippets: ClassMemberSnippets,
  #[serde(default)]
  pub object_literal_method_snippets: ObjectLiteralMethodSnippets,
}

impl Default for CompletionSettings {
  fn default() -> Self {
    Self {
      complete_function_calls: false,
      include_automatic_optional_chain_completions: true,
      include_completions_for_import_statements: true,
      names: true,
      paths: true,
      auto_imports: true,
      enabled: true,
      class_member_snippets: Default::default(),
      object_literal_method_snippets: Default::default(),
    }
  }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintsSettings {
  #[serde(default)]
  pub parameter_names: InlayHintsParamNamesOptions,
  #[serde(default)]
  pub parameter_types: InlayHintsParamTypesOptions,
  #[serde(default)]
  pub variable_types: InlayHintsVarTypesOptions,
  #[serde(default)]
  pub property_declaration_types: InlayHintsPropDeclTypesOptions,
  #[serde(default)]
  pub function_like_return_types: InlayHintsFuncLikeReturnTypesOptions,
  #[serde(default)]
  pub enum_member_values: InlayHintsEnumMemberValuesOptions,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintsParamNamesOptions {
  #[serde(default)]
  pub enabled: InlayHintsParamNamesEnabled,
  #[serde(default = "is_true")]
  pub suppress_when_argument_matches_name: bool,
}

impl Default for InlayHintsParamNamesOptions {
  fn default() -> Self {
    Self {
      enabled: InlayHintsParamNamesEnabled::None,
      suppress_when_argument_matches_name: true,
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum InlayHintsParamNamesEnabled {
  None,
  Literals,
  All,
}

impl Default for InlayHintsParamNamesEnabled {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintsParamTypesOptions {
  #[serde(default)]
  pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintsVarTypesOptions {
  #[serde(default)]
  pub enabled: bool,
  #[serde(default = "is_true")]
  pub suppress_when_type_matches_name: bool,
}

impl Default for InlayHintsVarTypesOptions {
  fn default() -> Self {
    Self {
      enabled: false,
      suppress_when_type_matches_name: true,
    }
  }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintsPropDeclTypesOptions {
  #[serde(default)]
  pub enabled: bool,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintsFuncLikeReturnTypesOptions {
  #[serde(default)]
  pub enabled: bool,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintsEnumMemberValuesOptions {
  #[serde(default)]
  pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImportCompletionSettings {
  /// A flag that indicates if non-explicitly set origins should be checked for
  /// supporting import suggestions.
  #[serde(default = "is_true")]
  pub auto_discover: bool,
  /// A map of origins which have had explicitly set if import suggestions are
  /// enabled.
  #[serde(default)]
  pub hosts: HashMap<String, bool>,
}

impl Default for ImportCompletionSettings {
  fn default() -> Self {
    Self {
      auto_discover: true,
      hosts: HashMap::default(),
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TestingSettings {
  /// A vector of arguments which should be used when running the tests for
  /// a workspace.
  #[serde(default)]
  pub args: Vec<String>,
}

impl Default for TestingSettings {
  fn default() -> Self {
    Self {
      args: vec!["--allow-all".to_string(), "--no-check".to_string()],
    }
  }
}

fn default_to_true() -> bool {
  true
}

fn default_document_preload_limit() -> usize {
  1000
}

fn empty_string_none<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
  let o: Option<String> = Option::deserialize(d)?;
  Ok(o.filter(|s| !s.is_empty()))
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ImportModuleSpecifier {
  NonRelative,
  ProjectRelative,
  Relative,
  Shortest,
}

impl Default for ImportModuleSpecifier {
  fn default() -> Self {
    Self::Shortest
  }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum JsxAttributeCompletionStyle {
  Auto,
  Braces,
  None,
}

impl Default for JsxAttributeCompletionStyle {
  fn default() -> Self {
    Self::Auto
  }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum QuoteStyle {
  Auto,
  Double,
  Single,
}

impl Default for QuoteStyle {
  fn default() -> Self {
    Self::Auto
  }
}

impl From<&FmtOptionsConfig> for QuoteStyle {
  fn from(config: &FmtOptionsConfig) -> Self {
    match config.single_quote {
      Some(true) => QuoteStyle::Single,
      _ => QuoteStyle::Double,
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LanguagePreferences {
  #[serde(default)]
  pub import_module_specifier: ImportModuleSpecifier,
  #[serde(default)]
  pub jsx_attribute_completion_style: JsxAttributeCompletionStyle,
  #[serde(default)]
  pub auto_import_file_exclude_patterns: Vec<String>,
  #[serde(default = "is_true")]
  pub use_aliases_for_renames: bool,
  #[serde(default)]
  pub quote_style: QuoteStyle,
}

impl Default for LanguagePreferences {
  fn default() -> Self {
    LanguagePreferences {
      import_module_specifier: Default::default(),
      jsx_attribute_completion_style: Default::default(),
      auto_import_file_exclude_patterns: vec![],
      use_aliases_for_renames: true,
      quote_style: Default::default(),
    }
  }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateImportsOnFileMoveOptions {
  #[serde(default)]
  pub enabled: UpdateImportsOnFileMoveEnabled,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum UpdateImportsOnFileMoveEnabled {
  Always,
  Prompt,
  Never,
}

impl Default for UpdateImportsOnFileMoveEnabled {
  fn default() -> Self {
    Self::Prompt
  }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LanguageWorkspaceSettings {
  #[serde(default)]
  pub inlay_hints: InlayHintsSettings,
  #[serde(default)]
  pub preferences: LanguagePreferences,
  #[serde(default)]
  pub suggest: CompletionSettings,
  #[serde(default)]
  pub update_imports_on_file_move: UpdateImportsOnFileMoveOptions,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum InspectSetting {
  Bool(bool),
  String(String),
}

impl Default for InspectSetting {
  fn default() -> Self {
    InspectSetting::Bool(false)
  }
}

impl InspectSetting {
  pub fn to_address(&self) -> Option<String> {
    match self {
      InspectSetting::Bool(false) => None,
      InspectSetting::Bool(true) => Some("127.0.0.1:9222".to_string()),
      InspectSetting::String(s) => Some(s.clone()),
    }
  }
}

/// Deno language server specific settings that are applied to a workspace.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSettings {
  /// A flag that indicates if Deno is enabled for the workspace.
  pub enable: Option<bool>,

  /// A list of paths, using the root_uri as a base that should be Deno
  /// disabled.
  #[serde(default)]
  pub disable_paths: Vec<String>,

  /// A list of paths, using the root_uri as a base that should be Deno enabled.
  pub enable_paths: Option<Vec<String>>,

  /// An option that points to a path string of the path to utilise as the
  /// cache/DENO_DIR for the language server.
  #[serde(default, deserialize_with = "empty_string_none")]
  pub cache: Option<String>,

  /// Cache local modules and their dependencies on `textDocument/didSave`
  /// notifications corresponding to them.
  #[serde(default)]
  pub cache_on_save: bool,

  /// Override the default stores used to validate certificates. This overrides
  /// the environment variable `DENO_TLS_CA_STORE` if present.
  pub certificate_stores: Option<Vec<String>>,

  /// An option that points to a path string of the config file to apply to
  /// code within the workspace.
  #[serde(default, deserialize_with = "empty_string_none")]
  pub config: Option<String>,

  /// An option that points to a path string of the import map to apply to the
  /// code within the workspace.
  #[serde(default, deserialize_with = "empty_string_none")]
  pub import_map: Option<String>,

  /// Code lens specific settings for the workspace.
  #[serde(default)]
  pub code_lens: CodeLensSettings,

  /// A flag that indicates if internal debug logging should be made available.
  #[serde(default)]
  pub internal_debug: bool,

  #[serde(default)]
  pub internal_inspect: InspectSetting,

  /// Write logs to a file in a project-local directory.
  #[serde(default)]
  pub log_file: bool,

  /// A flag that indicates if linting is enabled for the workspace.
  #[serde(default = "default_to_true")]
  pub lint: bool,

  /// Limits the number of files that can be preloaded by the language server.
  #[serde(default = "default_document_preload_limit")]
  pub document_preload_limit: usize,

  #[serde(default)]
  pub suggest: DenoCompletionSettings,

  /// Testing settings for the workspace.
  #[serde(default)]
  pub testing: TestingSettings,

  /// An option which sets the cert file to use when attempting to fetch remote
  /// resources. This overrides `DENO_CERT` if present.
  #[serde(default, deserialize_with = "empty_string_none")]
  pub tls_certificate: Option<String>,

  /// An option, if set, will unsafely ignore certificate errors when fetching
  /// remote resources.
  #[serde(default)]
  pub unsafely_ignore_certificate_errors: Option<Vec<String>>,

  #[serde(default)]
  pub unstable: bool,

  #[serde(default)]
  pub javascript: LanguageWorkspaceSettings,

  #[serde(default)]
  pub typescript: LanguageWorkspaceSettings,
}

impl Default for WorkspaceSettings {
  fn default() -> Self {
    WorkspaceSettings {
      enable: None,
      disable_paths: vec![],
      enable_paths: None,
      cache: None,
      cache_on_save: false,
      certificate_stores: None,
      config: None,
      import_map: None,
      code_lens: Default::default(),
      internal_debug: false,
      internal_inspect: Default::default(),
      log_file: false,
      lint: true,
      document_preload_limit: default_document_preload_limit(),
      suggest: Default::default(),
      testing: Default::default(),
      tls_certificate: None,
      unsafely_ignore_certificate_errors: None,
      unstable: false,
      javascript: Default::default(),
      typescript: Default::default(),
    }
  }
}

impl WorkspaceSettings {
  pub fn from_raw_settings(
    deno: Value,
    javascript: Value,
    typescript: Value,
  ) -> Self {
    fn parse_or_default<T: Default + DeserializeOwned>(
      value: Value,
      description: &str,
    ) -> T {
      if value.is_null() {
        return T::default();
      }
      match serde_json::from_value(value) {
        Ok(v) => v,
        Err(err) => {
          lsp_warn!("Couldn't parse {description}: {err}");
          T::default()
        }
      }
    }
    let deno_inlay_hints = deno.as_object().and_then(|o| o.get("inlayHints").cloned());
    let deno_suggest = deno.as_object().and_then(|o| o.get("suggest").cloned());
    let mut settings: Self = parse_or_default(deno, "settings under \"deno\"");
    settings.javascript = parse_or_default(javascript, "settings under \"javascript\"");
    settings.typescript = parse_or_default(typescript, "settings under \"typescript\"");
    if let Some(inlay_hints) = deno_inlay_hints {
      let inlay_hints: InlayHintsSettings =
        parse_or_default(inlay_hints, "settings under \"deno.inlayHints\"");
      if inlay_hints.parameter_names.enabled != Default::default() {
        lsp_warn!("\"deno.inlayHints.parameterNames.enabled\" is deprecated. Instead use \"javascript.inlayHints.parameterNames.enabled\" and \"typescript.inlayHints.parameterNames.enabled\".");
        settings.javascript.inlay_hints.parameter_names.enabled =
          inlay_hints.parameter_names.enabled.clone();
        settings.typescript.inlay_hints.parameter_names.enabled =
          inlay_hints.parameter_names.enabled;
      }
      if !inlay_hints
        .parameter_names
        .suppress_when_argument_matches_name
      {
        lsp_warn!("\"deno.inlayHints.parameterNames.suppressWhenArgumentMatchesName\" is deprecated. Instead use \"javascript.inlayHints.parameterNames.suppressWhenArgumentMatchesName\" and \"typescript.inlayHints.parameterNames.suppressWhenArgumentMatchesName\".");
        settings
          .javascript
          .inlay_hints
          .parameter_names
          .suppress_when_argument_matches_name = inlay_hints
          .parameter_names
          .suppress_when_argument_matches_name;
        settings
          .typescript
          .inlay_hints
          .parameter_names
          .suppress_when_argument_matches_name = inlay_hints
          .parameter_names
          .suppress_when_argument_matches_name;
      }
      if inlay_hints.parameter_types.enabled {
        lsp_warn!("\"deno.inlayHints.parameterTypes.enabled\" is deprecated. Instead use \"javascript.inlayHints.parameterTypes.enabled\" and \"typescript.inlayHints.parameterTypes.enabled\".");
        settings.javascript.inlay_hints.parameter_types.enabled =
          inlay_hints.parameter_types.enabled;
        settings.typescript.inlay_hints.parameter_types.enabled =
          inlay_hints.parameter_types.enabled;
      }
      if inlay_hints.variable_types.enabled {
        lsp_warn!("\"deno.inlayHints.variableTypes.enabled\" is deprecated. Instead use \"javascript.inlayHints.variableTypes.enabled\" and \"typescript.inlayHints.variableTypes.enabled\".");
        settings.javascript.inlay_hints.variable_types.enabled = inlay_hints.variable_types.enabled;
        settings.typescript.inlay_hints.variable_types.enabled = inlay_hints.variable_types.enabled;
      }
      if !inlay_hints.variable_types.suppress_when_type_matches_name {
        lsp_warn!("\"deno.inlayHints.variableTypes.suppressWhenTypeMatchesName\" is deprecated. Instead use \"javascript.inlayHints.variableTypes.suppressWhenTypeMatchesName\" and \"typescript.inlayHints.variableTypes.suppressWhenTypeMatchesName\".");
        settings
          .javascript
          .inlay_hints
          .variable_types
          .suppress_when_type_matches_name =
          inlay_hints.variable_types.suppress_when_type_matches_name;
        settings
          .typescript
          .inlay_hints
          .variable_types
          .suppress_when_type_matches_name =
          inlay_hints.variable_types.suppress_when_type_matches_name;
      }
      if inlay_hints.property_declaration_types.enabled {
        lsp_warn!("\"deno.inlayHints.propertyDeclarationTypes.enabled\" is deprecated. Instead use \"javascript.inlayHints.propertyDeclarationTypes.enabled\" and \"typescript.inlayHints.propertyDeclarationTypes.enabled\".");
        settings
          .javascript
          .inlay_hints
          .property_declaration_types
          .enabled = inlay_hints.property_declaration_types.enabled;
        settings
          .typescript
          .inlay_hints
          .property_declaration_types
          .enabled = inlay_hints.property_declaration_types.enabled;
      }
      if inlay_hints.function_like_return_types.enabled {
        lsp_warn!("\"deno.inlayHints.functionLikeReturnTypes.enabled\" is deprecated. Instead use \"javascript.inlayHints.functionLikeReturnTypes.enabled\" and \"typescript.inlayHints.functionLikeReturnTypes.enabled\".");
        settings
          .javascript
          .inlay_hints
          .function_like_return_types
          .enabled = inlay_hints.function_like_return_types.enabled;
        settings
          .typescript
          .inlay_hints
          .function_like_return_types
          .enabled = inlay_hints.function_like_return_types.enabled;
      }
      if inlay_hints.enum_member_values.enabled {
        lsp_warn!("\"deno.inlayHints.enumMemberValues.enabled\" is deprecated. Instead use \"javascript.inlayHints.enumMemberValues.enabled\" and \"typescript.inlayHints.enumMemberValues.enabled\".");
        settings.javascript.inlay_hints.enum_member_values.enabled =
          inlay_hints.enum_member_values.enabled;
        settings.typescript.inlay_hints.enum_member_values.enabled =
          inlay_hints.enum_member_values.enabled;
      }
    }
    if let Some(suggest) = deno_suggest {
      let suggest: CompletionSettings =
        parse_or_default(suggest, "settings under \"deno.suggest\"");
      if suggest.complete_function_calls {
        lsp_warn!("\"deno.suggest.completeFunctionCalls\" is deprecated. Instead use \"javascript.suggest.completeFunctionCalls\" and \"typescript.suggest.completeFunctionCalls\".");
        settings.javascript.suggest.complete_function_calls = suggest.complete_function_calls;
        settings.typescript.suggest.complete_function_calls = suggest.complete_function_calls;
      }
      if !suggest.names {
        lsp_warn!("\"deno.suggest.names\" is deprecated. Instead use \"javascript.suggest.names\" and \"typescript.suggest.names\".");
        settings.javascript.suggest.names = suggest.names;
        settings.typescript.suggest.names = suggest.names;
      }
      if !suggest.paths {
        lsp_warn!("\"deno.suggest.paths\" is deprecated. Instead use \"javascript.suggest.paths\" and \"typescript.suggest.paths\".");
        settings.javascript.suggest.paths = suggest.paths;
        settings.typescript.suggest.paths = suggest.paths;
      }
      if !suggest.auto_imports {
        lsp_warn!("\"deno.suggest.autoImports\" is deprecated. Instead use \"javascript.suggest.autoImports\" and \"typescript.suggest.autoImports\".");
        settings.javascript.suggest.auto_imports = suggest.auto_imports;
        settings.typescript.suggest.auto_imports = suggest.auto_imports;
      }
    }
    settings
  }

  pub fn from_initialization_options(options: Value) -> Self {
    let deno = options;
    let javascript = deno
      .as_object()
      .and_then(|o| o.get("javascript").cloned())
      .unwrap_or_default();
    let typescript = deno
      .as_object()
      .and_then(|o| o.get("typescript").cloned())
      .unwrap_or_default();
    Self::from_raw_settings(deno, javascript, typescript)
  }
}

#[derive(Debug, Clone, Default)]
pub struct ConfigSnapshot {
  pub client_capabilities: ClientCapabilities,
  pub config_file: Option<ConfigFile>,
  pub settings: Settings,
  pub workspace_folders: Vec<(ModuleSpecifier, lsp::WorkspaceFolder)>,
}

impl ConfigSnapshot {
  pub fn workspace_settings_for_specifier(
    &self,
    specifier: &ModuleSpecifier,
  ) -> &WorkspaceSettings {
    self.settings.get_for_specifier(specifier).0
  }

  /// Determine if the provided specifier is enabled or not.
  pub fn specifier_enabled(
    &self,
    specifier: &ModuleSpecifier,
  ) -> bool {
    specifier_enabled(
      specifier,
      self.config_file.as_ref(),
      &self.settings,
      &self.workspace_folders,
    )
  }

  pub fn specifier_enabled_for_test(
    &self,
    specifier: &ModuleSpecifier,
  ) -> bool {
    if let Some(cf) = &self.config_file {
      if let Some(options) = cf.to_test_config().ok().flatten() {
        if !options.files.matches_specifier(specifier) {
          return false;
        }
      }
    }
    self.specifier_enabled(specifier)
  }
}

#[derive(Debug, Default, Clone)]
pub struct Settings {
  pub unscoped: WorkspaceSettings,
  pub by_workspace_folder: Option<BTreeMap<ModuleSpecifier, WorkspaceSettings>>,
}

impl Settings {
  pub fn get_unscoped(&self) -> &WorkspaceSettings {
    &self.unscoped
  }

  pub fn get_for_specifier(
    &self,
    specifier: &ModuleSpecifier,
  ) -> (&WorkspaceSettings, Option<&ModuleSpecifier>) {
    let Ok(path) = specifier_to_file_path(specifier) else {
      return (&self.unscoped, None);
    };
    if let Some(by_workspace_folder) = &self.by_workspace_folder {
      for (folder_uri, settings) in by_workspace_folder.iter().rev() {
        let Ok(folder_path) = specifier_to_file_path(folder_uri) else {
          continue;
        };
        if path.starts_with(folder_path) {
          return (settings, Some(folder_uri));
        }
      }
    }
    (&self.unscoped, None)
  }

  pub fn set_unscoped(
    &mut self,
    mut settings: WorkspaceSettings,
  ) {
    // See https://github.com/denoland/vscode_deno/issues/908.
    if settings.enable_paths == Some(vec![]) {
      settings.enable_paths = None;
    }
    self.unscoped = settings;
  }

  pub fn set_for_workspace_folders(
    &mut self,
    mut by_workspace_folder: Option<BTreeMap<ModuleSpecifier, WorkspaceSettings>>,
  ) {
    if let Some(by_workspace_folder) = &mut by_workspace_folder {
      for settings in by_workspace_folder.values_mut() {
        // See https://github.com/denoland/vscode_deno/issues/908.
        if settings.enable_paths == Some(vec![]) {
          settings.enable_paths = None;
        }
      }
    }
    self.by_workspace_folder = by_workspace_folder;
  }
}

#[derive(Debug)]
struct WithCanonicalizedSpecifier<T> {
  /// Stored canonicalized specifier, which is used for file watcher events.
  canonicalized_specifier: ModuleSpecifier,
  file: T,
}

/// Contains the config file and dependent information.
#[derive(Debug)]
struct LspConfigFileInfo {
  config_file: WithCanonicalizedSpecifier<ConfigFile>,
  /// An optional deno.lock file, which is resolved relative to the config file.
  maybe_lockfile: Option<WithCanonicalizedSpecifier<Arc<Mutex<Lockfile>>>>,
  /// The canonicalized node_modules directory, which is found relative to the config file.
  maybe_node_modules_dir: Option<PathBuf>,
}

#[derive(Debug)]
pub struct Config {
  pub client_capabilities: ClientCapabilities,
  settings: Settings,
  pub workspace_folders: Vec<(ModuleSpecifier, lsp::WorkspaceFolder)>,
  /// An optional configuration file which has been specified in the client
  /// options along with some data that is computed after the config file is set.
  maybe_config_file_info: Option<LspConfigFileInfo>,
}

impl Config {
  pub fn new() -> Self {
    Self {
      client_capabilities: ClientCapabilities::default(),
      // Root provided by the initialization parameters.
      settings: Default::default(),
      workspace_folders: vec![],
      maybe_config_file_info: None,
    }
  }

  pub fn set_workspace_settings(
    &mut self,
    unscoped: WorkspaceSettings,
    by_workspace_folder: Option<BTreeMap<ModuleSpecifier, WorkspaceSettings>>,
  ) {
    self.settings.set_unscoped(unscoped);
    self.settings.set_for_workspace_folders(by_workspace_folder);
  }

  pub fn workspace_settings(&self) -> &WorkspaceSettings {
    self.settings.get_unscoped()
  }

  pub fn workspace_settings_for_specifier(
    &self,
    specifier: &ModuleSpecifier,
  ) -> &WorkspaceSettings {
    self.settings.get_for_specifier(specifier).0
  }

  pub fn language_settings_for_specifier(
    &self,
    specifier: &ModuleSpecifier,
  ) -> Option<&LanguageWorkspaceSettings> {
    let workspace_settings = self.workspace_settings_for_specifier(specifier);
    match MediaType::from_specifier(specifier) {
      MediaType::JavaScript | MediaType::Jsx | MediaType::Mjs | MediaType::Cjs => {
        Some(&workspace_settings.javascript)
      }
      MediaType::TypeScript
      | MediaType::Mts
      | MediaType::Cts
      | MediaType::Dts
      | MediaType::Dmts
      | MediaType::Dcts
      | MediaType::Tsx => Some(&workspace_settings.typescript),
      MediaType::Json
      | MediaType::Wasm
      | MediaType::TsBuildInfo
      | MediaType::SourceMap
      | MediaType::Unknown => None,
    }
  }

  /// Determine if any inlay hints are enabled. This allows short circuiting
  /// when there are no inlay hints enabled.
  pub fn enabled_inlay_hints_for_specifier(
    &self,
    specifier: &ModuleSpecifier,
  ) -> bool {
    let Some(settings) = self.language_settings_for_specifier(specifier) else {
      return false;
    };
    !matches!(
      settings.inlay_hints.parameter_names.enabled,
      InlayHintsParamNamesEnabled::None
    ) || settings.inlay_hints.parameter_types.enabled
      || settings.inlay_hints.variable_types.enabled
      || settings.inlay_hints.property_declaration_types.enabled
      || settings.inlay_hints.function_like_return_types.enabled
      || settings.inlay_hints.enum_member_values.enabled
  }

  pub fn root_uri(&self) -> Option<&Url> {
    self.workspace_folders.first().map(|p| &p.0)
  }

  pub fn maybe_node_modules_dir_path(&self) -> Option<&PathBuf> {
    self
      .maybe_config_file_info
      .as_ref()
      .and_then(|p| p.maybe_node_modules_dir.as_ref())
  }

  pub fn maybe_vendor_dir_path(&self) -> Option<PathBuf> {
    self.maybe_config_file().and_then(|c| c.vendor_dir_path())
  }

  pub fn maybe_config_file(&self) -> Option<&ConfigFile> {
    self
      .maybe_config_file_info
      .as_ref()
      .map(|c| &c.config_file.file)
  }

  /// Canonicalized specifier of the config file, which should only be used for
  /// file watcher events. Otherwise, prefer using the non-canonicalized path
  /// as the rest of the CLI does for config files.
  pub fn maybe_config_file_canonicalized_specifier(&self) -> Option<&ModuleSpecifier> {
    self
      .maybe_config_file_info
      .as_ref()
      .map(|c| &c.config_file.canonicalized_specifier)
  }

  pub fn maybe_lockfile(&self) -> Option<&Arc<Mutex<Lockfile>>> {
    self
      .maybe_config_file_info
      .as_ref()
      .and_then(|c| c.maybe_lockfile.as_ref().map(|l| &l.file))
  }

  /// Canonicalized specifier of the lockfile, which should only be used for
  /// file watcher events. Otherwise, prefer using the non-canonicalized path
  /// as the rest of the CLI does for config files.
  pub fn maybe_lockfile_canonicalized_specifier(&self) -> Option<&ModuleSpecifier> {
    self.maybe_config_file_info.as_ref().and_then(|c| {
      c.maybe_lockfile
        .as_ref()
        .map(|l| &l.canonicalized_specifier)
    })
  }

  pub fn clear_config_file(&mut self) {
    self.maybe_config_file_info = None;
  }

  pub fn has_config_file(&self) -> bool {
    self.maybe_config_file_info.is_some()
  }

  pub fn set_config_file(
    &mut self,
    config_file: ConfigFile,
  ) {
    self.maybe_config_file_info = Some(LspConfigFileInfo {
      maybe_lockfile: resolve_lockfile_from_config(&config_file).map(|lockfile| {
        let path = canonicalize_path_maybe_not_exists(&lockfile.filename)
          .unwrap_or_else(|_| lockfile.filename.clone());
        WithCanonicalizedSpecifier {
          canonicalized_specifier: ModuleSpecifier::from_file_path(path).unwrap(),
          file: Arc::new(Mutex::new(lockfile)),
        }
      }),
      maybe_node_modules_dir: resolve_node_modules_dir(&config_file),
      config_file: WithCanonicalizedSpecifier {
        canonicalized_specifier: config_file
          .specifier
          .to_file_path()
          .ok()
          .and_then(|p| canonicalize_path_maybe_not_exists(&p).ok())
          .and_then(|p| ModuleSpecifier::from_file_path(p).ok())
          .unwrap_or_else(|| config_file.specifier.clone()),
        file: config_file,
      },
    });
  }

  pub fn snapshot(&self) -> Arc<ConfigSnapshot> {
    Arc::new(ConfigSnapshot {
      client_capabilities: self.client_capabilities.clone(),
      config_file: self.maybe_config_file().cloned(),
      settings: self.settings.clone(),
      workspace_folders: self.workspace_folders.clone(),
    })
  }

  pub fn specifier_enabled(
    &self,
    specifier: &ModuleSpecifier,
  ) -> bool {
    specifier_enabled(
      specifier,
      self.maybe_config_file(),
      &self.settings,
      &self.workspace_folders,
    )
  }

  pub fn specifier_enabled_for_test(
    &self,
    specifier: &ModuleSpecifier,
  ) -> bool {
    if let Some(cf) = self.maybe_config_file() {
      if let Some(options) = cf.to_test_config().ok().flatten() {
        if !options.files.matches_specifier(specifier) {
          return false;
        }
      }
    }
    if !self.specifier_enabled(specifier) {
      return false;
    }
    true
  }

  pub fn get_enabled_paths(&self) -> PathOrPatternSet {
    let mut paths = vec![];
    for (workspace_uri, _) in &self.workspace_folders {
      let Ok(workspace_path) = specifier_to_file_path(workspace_uri) else {
        lsp_log!("Unable to convert uri \"{}\" to path.", workspace_uri);
        continue;
      };
      let settings = self.workspace_settings_for_specifier(workspace_uri);
      if let Some(enable_paths) = &settings.enable_paths {
        for path in enable_paths {
          match PathOrPattern::from_relative(&workspace_path, path) {
            Ok(path_or_pattern) => paths.push(path_or_pattern),
            Err(err) => {
              lsp_log!("Invalid enable path '{}': {:#}", path, err);
            }
          }
        }
      } else {
        paths.push(PathOrPattern::Path(workspace_path));
      }
    }
    paths.sort();
    paths.dedup();
    PathOrPatternSet::new(paths)
  }

  pub fn get_disabled_paths(&self) -> PathOrPatternSet {
    let mut path_or_patterns = vec![];
    if let Some(cf) = self.maybe_config_file() {
      if let Ok(files) = cf.to_files_config() {
        for path in files.exclude.into_path_or_patterns() {
          path_or_patterns.push(path);
        }
      }
    }
    for (workspace_uri, _) in &self.workspace_folders {
      let Ok(workspace_path) = specifier_to_file_path(workspace_uri) else {
        lsp_log!("Unable to convert uri \"{}\" to path.", workspace_uri);
        continue;
      };
      let settings = self.workspace_settings_for_specifier(workspace_uri);
      let is_enabled = settings
        .enable_paths
        .as_ref()
        .map(|p| !p.is_empty())
        .unwrap_or_else(|| settings.enable.unwrap_or_else(|| self.has_config_file()));
      if is_enabled {
        for path in &settings.disable_paths {
          path_or_patterns.push(PathOrPattern::Path(workspace_path.join(path)));
        }
      } else {
        path_or_patterns.push(PathOrPattern::Path(workspace_path));
      }
    }
    path_or_patterns.sort();
    path_or_patterns.dedup();
    PathOrPatternSet::new(path_or_patterns)
  }

  pub fn log_file(&self) -> bool {
    self.settings.unscoped.log_file
  }

  pub fn internal_inspect(&self) -> &InspectSetting {
    &self.settings.unscoped.internal_inspect
  }

  pub fn update_capabilities(
    &mut self,
    capabilities: &lsp::ClientCapabilities,
  ) {
    if let Some(experimental) = &capabilities.experimental {
      self.client_capabilities.status_notification = experimental
        .get("statusNotification")
        .and_then(|it| it.as_bool())
        == Some(true);
      self.client_capabilities.testing_api =
        experimental.get("testingApi").and_then(|it| it.as_bool()) == Some(true);
    }

    if let Some(workspace) = &capabilities.workspace {
      self.client_capabilities.workspace_configuration = workspace.configuration.unwrap_or(false);
      self.client_capabilities.workspace_did_change_watched_files = workspace
        .did_change_watched_files
        .and_then(|it| it.dynamic_registration)
        .unwrap_or(false);
      if let Some(file_operations) = &workspace.file_operations {
        if let Some(true) = file_operations.dynamic_registration {
          self.client_capabilities.workspace_will_rename_files =
            file_operations.will_rename.unwrap_or(false);
        }
      }
    }

    if let Some(text_document) = &capabilities.text_document {
      self.client_capabilities.line_folding_only = text_document
        .folding_range
        .as_ref()
        .and_then(|it| it.line_folding_only)
        .unwrap_or(false);
      self.client_capabilities.code_action_disabled_support = text_document
        .code_action
        .as_ref()
        .and_then(|it| it.disabled_support)
        .unwrap_or(false);
      self.client_capabilities.snippet_support = if let Some(completion) = &text_document.completion
      {
        completion
          .completion_item
          .as_ref()
          .and_then(|it| it.snippet_support)
          .unwrap_or(false)
      } else {
        false
      };
    }
  }
}

fn specifier_enabled(
  specifier: &Url,
  config_file: Option<&ConfigFile>,
  settings: &Settings,
  workspace_folders: &[(Url, lsp::WorkspaceFolder)],
) -> bool {
  if let Some(cf) = config_file {
    if let Ok(files) = cf.to_files_config() {
      if !files.matches_specifier(specifier) {
        return false;
      }
    }
  }
  let Ok(path) = specifier_to_file_path(specifier) else {
    // Non-file URLs are not disabled by these settings.
    return true;
  };
  let (settings, mut folder_uri) = settings.get_for_specifier(specifier);
  folder_uri = folder_uri.or_else(|| workspace_folders.first().map(|f| &f.0));
  let mut disable_paths = vec![];
  let mut enable_paths = None;
  if let Some(folder_uri) = folder_uri {
    if let Ok(folder_path) = specifier_to_file_path(folder_uri) {
      disable_paths = settings
        .disable_paths
        .iter()
        .map(|p| folder_path.join(p))
        .collect::<Vec<_>>();
      enable_paths = settings.enable_paths.as_ref().map(|enable_paths| {
        enable_paths
          .iter()
          .map(|p| folder_path.join(p))
          .collect::<Vec<_>>()
      });
    }
  }
  if let Some(enable_paths) = &enable_paths {
    for enable_path in enable_paths {
      if path.starts_with(enable_path) && !disable_paths.iter().any(|p| path.starts_with(p)) {
        return true;
      }
    }
    false
  } else {
    settings.enable.unwrap_or_else(|| config_file.is_some())
      && !disable_paths.iter().any(|p| path.starts_with(p))
  }
}

fn resolve_lockfile_from_config(config_file: &ConfigFile) -> Option<Lockfile> {
  let lockfile_path = match config_file.resolve_lockfile_path() {
    Ok(Some(value)) => value,
    Ok(None) => return None,
    Err(err) => {
      lsp_warn!("Error resolving lockfile: {:#}", err);
      return None;
    }
  };
  resolve_lockfile_from_path(lockfile_path)
}

fn resolve_node_modules_dir(config_file: &ConfigFile) -> Option<PathBuf> {
  // For the language server, require an explicit opt-in via the
  // `nodeModulesDir: true` setting in the deno.json file. This is to
  // reduce the chance of modifying someone's node_modules directory
  // without them having asked us to do so.
  let explicitly_disabled = config_file.json.node_modules_dir == Some(false);
  if explicitly_disabled {
    return None;
  }
  let enabled =
    config_file.json.node_modules_dir == Some(true) || config_file.json.vendor == Some(true);
  if !enabled {
    return None;
  }
  if config_file.specifier.scheme() != "file" {
    return None;
  }
  let file_path = config_file.specifier.to_file_path().ok()?;
  let node_modules_dir = file_path.parent()?.join("node_modules");
  canonicalize_path_maybe_not_exists(&node_modules_dir).ok()
}

fn resolve_lockfile_from_path(lockfile_path: PathBuf) -> Option<Lockfile> {
  match Lockfile::new(lockfile_path, false) {
    Ok(value) => {
      if let Ok(specifier) = ModuleSpecifier::from_file_path(&value.filename) {
        lsp_log!("  Resolved lock file: \"{}\"", specifier);
      }
      Some(value)
    }
    Err(err) => {
      lsp_warn!("Error loading lockfile: {:#}", err);
      None
    }
  }
}
