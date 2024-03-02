//! https://github.com/drivasperez/tsconfig/blob/master/src/lib.rs
//!
//! A Rust crate for parsing TypeScript's TSConfig files into a struct.
//!
//! A TSConfig file in a directory indicates that the directory is the root of a TypeScript or JavaScript project.
//! The TSConfig file can be either a tsconfig.json or jsconfig.json; both have the same behavior and the same set of config variables.
//!
//! One TSConfig can inherit fields from another if it is specified in the 'extends' field.
//!
//! ## Example usage
//!
//! ```
//! use tsconfig::TsConfig;
//! use std::path::Path;
//!
//! let path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
//!     .join("test/tsconfig.default.json");
//! let config = TsConfig::parse_file(&path).unwrap();
//!
//! ```

use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

use json_comments::StripComments;
use regex::Regex;
use serde::Deserialize;
use serde::Deserializer;
use serde_json::Value;

use thiserror::Error;

pub type Result<T, E = ConfigError> = std::result::Result<T, E>;

/// Errors when parsing TsConfig files.
/// This is non-exhaustive, and may be extended in the future.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ConfigError {
  #[error("Could not parse configuration file")]
  ParseError(#[from] serde_json::Error),
  #[error("Could not read file")]
  CouldNotFindFile(#[from] std::io::Error),
  #[error("Could not convert path into UTF-8: {0}")]
  InvalidPath(String),
}

/// The main struct representing a parsed .tsconfig file.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TsConfig {
  pub exclude: Option<Vec<String>>,
  pub extends: Option<String>,
  pub files: Option<Vec<String>>,
  pub include: Option<Vec<String>>,
  pub references: Option<References>,
  pub type_acquisition: Option<TypeAcquisition>,
  pub compiler_options: Option<CompilerOptions>,
}

impl TsConfig {
  /// Parses a .tsconfig file into a [TsConfig].
  ///
  /// The `extends` field will be respected, allowing for one .tsconfig file to inherit properties from another.
  /// Comments and trailing commas are both allowed, although they are not valid JSON.
  /// ## Example
  ///
  /// Assuming the following .tsconfig files:
  ///
  /// tsconfig.base.json:
  /// ```json
  /// {
  ///     "useDefineForClassFields": false,
  ///     "traceResolution": true,
  ///     "jsx": "preserve",
  /// }
  /// ```
  /// tsconfig.inherits.json:
  /// ```json
  /// {
  ///     "extends": "./tsconfig.base.json",
  ///     "compilerOptions": {
  ///         "traceResolution": false,
  ///         "declaration": true,
  ///         "jsx": "react-jsxdev",
  ///     }
  /// }
  /// ```
  /// ```
  /// use std::path::Path;
  /// use tsconfig::TsConfig;
  /// let path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
  ///     .join("test/tsconfig.inherits.json");
  /// let config = TsConfig::parse_file(&path).unwrap();
  ///
  /// assert_eq!(
  ///     config
  ///         .compiler_options
  ///         .clone()
  ///         .unwrap()
  ///         .use_define_for_class_fields,
  ///     Some(false)
  /// );
  ///
  /// assert_eq!(
  ///     config.compiler_options.clone().unwrap().declaration,
  ///     Some(true)
  /// );
  ///
  /// assert_eq!(
  ///     config.compiler_options.unwrap().trace_resolution,
  ///     Some(false)
  /// );
  ///
  /// ```
  pub fn parse_file<P: AsRef<Path>>(path: &P) -> Result<TsConfig> {
    let values = parse_file_to_value(path)?;
    let cfg = serde_json::from_value(values)?;
    Ok(cfg)
  }

  /// Parse a JSON string into a single [TsConfig].
  ///
  /// The 'extends' field will be ignored. Comments and trailing commas are both allowed, although they are not valid JSON.
  ///
  /// ## Example
  /// ```
  /// use tsconfig::{TsConfig, Jsx};
  /// let json = r#"{"compilerOptions": {"jsx": /*here's a comment*/ "react-jsx"},}"#;
  ///
  /// let config = TsConfig::parse_str(json).unwrap();
  /// assert_eq!(config.compiler_options.unwrap().jsx, Some(Jsx::ReactJsx));     
  ///```
  ///
  pub fn parse_str(json: &str) -> Result<TsConfig> {
    // Remove trailing commas from objects.
    let re = Regex::new(r",(?P<valid>\s*})").unwrap();
    let mut stripped = String::with_capacity(json.len());
    StripComments::new(json.as_bytes()).read_to_string(&mut stripped)?;
    let stripped = re.replace_all(&stripped, "$valid");
    let r: TsConfig = serde_json::from_str(&stripped)?;
    Ok(r)
  }
}

fn merge(
  a: &mut Value,
  b: Value,
) {
  match (a, b) {
    (&mut Value::Object(ref mut a), Value::Object(b)) => {
      for (k, v) in b {
        merge(a.entry(k).or_insert(Value::Null), v);
      }
    }
    (a, b) => {
      if let Value::Null = a {
        *a = b;
      }
    }
  }
}

/// Parses a .tsconfig file into a [serde_json::Value].
///
/// The `extends` field will be respected, allowing for one .tsconfig file to inherit properties from another.
/// Comments and trailing commas are both allowed, although they are not valid JSON.
/// ## Example
///
/// Assuming the following .tsconfig files:
///
/// tsconfig.base.json:
/// ```json
/// {
///     "compilerOptions": {
///         "useDefineForClassFields": false,
///         "traceResolution": true,
///         "jsx": "preserve",
///     }
/// }
/// ```
/// tsconfig.inherits.json:
/// ```json
/// {
///     "extends": "./tsconfig.base.json",
///     "compilerOptions": {
///         "traceResolution": false,
///         "declaration": true,
///         "jsx": "react-jsxdev",
///     }
/// }
/// ```
/// ```
/// use std::path::Path;
/// use tsconfig::parse_file_to_value;
/// use serde_json::Value;
///
/// let path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
///     .join("test/tsconfig.inherits.json");
/// let config = parse_file_to_value(&path).unwrap();
///
/// assert_eq!(
///     config
///         ["compilerOptions"]
///         ["useDefineForClassFields"],
///     Value::Bool(false)
/// );
///
///
/// ```
pub fn parse_file_to_value<P: AsRef<Path>>(path: &P) -> Result<Value> {
  let s = std::fs::read_to_string(path)?;
  let mut value = parse_to_value(&s)?;

  if let Value::String(s) = &value["extends"] {
    // This may or may not have a `.json` extension
    let extends_path_unchecked = path
      .as_ref()
      .parent()
      .unwrap_or_else(|| Path::new(""))
      .join(s);

    let extends_path_str = extends_path_unchecked.to_str().ok_or_else(|| {
      ConfigError::InvalidPath(extends_path_unchecked.to_string_lossy().to_string())
    })?;

    // Append the extension if it doesn't already have it
    let extends_path = if extends_path_str.ends_with(&".json") {
      extends_path_unchecked
    } else {
      let with_ext = extends_path_str.to_string() + ".json";
      Path::new(with_ext.as_str()).to_path_buf()
    };
    let extends_value = parse_file_to_value(&extends_path)?;
    merge(&mut value, extends_value);
  }

  Ok(value)
}

/// Parse a JSON string into a single [serde_json::Value].
///
/// The 'extends' field will be ignored. Comments and trailing commas are both allowed, although they are not valid JSON.
///
/// ## Example
/// ```
/// use tsconfig::parse_to_value;
/// use serde_json::Value;
///
///
/// let json = r#"{"compilerOptions": {"jsx": /*here's a comment*/ "react-jsx"},}"#;
///
/// let config = parse_to_value(json).unwrap();
/// assert_eq!(config["compilerOptions"]["jsx"], Value::String("react-jsx".to_string()));     
///```
///
pub fn parse_to_value(json: &str) -> Result<Value> {
  // Remove trailing commas from objects.
  let re = Regex::new(r",(?P<valid>\s*})").unwrap();
  let mut stripped = String::with_capacity(json.len());
  StripComments::new(json.as_bytes()).read_to_string(&mut stripped)?;
  let stripped = re.replace_all(&stripped, "$valid");
  let r: Value = serde_json::from_str(&stripped)?;
  Ok(r)
}

/// Project references setting  
///
/// Project references are a way to structure your TypeScript programs into smaller pieces. Using
/// Project References can greatly improve build and editor interaction times, enforce logical separation
/// between components, and organize your code in new and improved ways.
///
/// You can read more about how references works in the Project References section of [the handbook](https://www.typescriptlang.org/docs/handbook/project-references.html).
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum References {
  Bool(bool),
  References(Vec<Reference>),
}

/// Project references setting  
///
/// Project references are a way to structure your TypeScript programs into smaller pieces. Using
/// Project References can greatly improve build and editor interaction times, enforce logical separation
/// between components, and organize your code in new and improved ways.
///
/// You can read more about how references works in the Project References section of [the handbook](https://www.typescriptlang.org/docs/handbook/project-references.html).
#[derive(Deserialize, Debug, Clone)]
pub struct Reference {
  pub path: String,
  pub prepend: Option<bool>,
}

/// Defines how automatic type acquisition behaves.
///
/// When you have a JavaScript project in your editor, TypeScript will provide types for your node_modules automatically
/// using the DefinitelyTyped set of @types definitions. This is called automatic type acquisition, and you can customize
/// it using the typeAcquisition object in your configuration.
///
/// If you would like to disable or customize this feature, create a jsconfig.json in the root of your project:
///
/// ```json
/// {
///   "typeAcquisition": {
///     "enable": false
///   }
/// }
/// ```
///
/// If you have a specific module which should be included (but isn’t in node_modules):
///
/// ```json
/// {
///   "typeAcquisition": {
///     "include": ["jest"]
///   }
/// }
/// ```
///
/// If a module should not be automatically acquired, for example if the library is available in your node_modules but your team has agreed to not use it:
///
/// ```json
/// {
///   "typeAcquisition": {
///     "exclude": ["jquery"]
///   }
/// }
/// ```
///
/// In TypeScript 4.1, we added the ability to disable the special-casing where a filename would trigger type acquisition:
///
/// ```json
/// {
///   "typeAcquisition": {
///     "disableFilenameBasedTypeAcquisition": true
///   }
/// }
/// ```
///
/// This means that having a file like jquery.js in your project would not automatically download the types for JQuery from DefinitelyTyped.
///
#[derive(Deserialize, Debug, Clone)]
pub enum TypeAcquisition {
  Bool(bool),
  Object {
    enable: bool,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    disable_filename_based_type_acquisition: Option<bool>,
  },
}

/// These options make up the bulk of TypeScript’s configuration and it covers how the language should work.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
  pub allow_js: Option<bool>,
  pub check_js: Option<bool>,
  pub composite: Option<bool>,
  pub declaration: Option<bool>,
  pub declaration_map: Option<bool>,
  pub downlevel_iteration: Option<bool>,
  pub import_helpers: Option<bool>,
  pub incremental: Option<bool>,
  pub isolated_modules: Option<bool>,
  pub jsx: Option<Jsx>,
  pub lib: Option<Vec<Lib>>,
  pub module: Option<Module>,
  pub no_emit: Option<bool>,
  pub out_dir: Option<String>,
  pub out_file: Option<String>,
  pub remove_comments: Option<bool>,
  pub root_dir: Option<String>,
  pub source_map: Option<bool>,
  pub target: Option<Target>,
  pub ts_build_info_file: Option<String>,
  pub always_strict: Option<bool>,
  pub no_implicit_any: Option<bool>,
  pub no_implicit_this: Option<bool>,
  pub strict: Option<bool>,
  pub strict_bind_call_apply: Option<bool>,
  pub strict_function_types: Option<bool>,
  pub strict_null_checks: Option<bool>,
  pub strict_property_initialization: Option<bool>,
  pub allow_synthetic_default_imports: Option<bool>,
  pub allow_umd_global_access: Option<bool>,
  pub base_url: Option<String>,
  pub es_module_interop: Option<bool>,
  pub module_resolution: Option<ModuleResolutionMode>,
  pub paths: Option<HashMap<String, Vec<String>>>,
  pub preserve_symlinks: Option<bool>,
  pub root_dirs: Option<Vec<String>>,
  pub type_roots: Option<Vec<String>>,
  pub types: Option<Vec<String>>,
  pub inline_source_map: Option<bool>,
  pub inline_sources: Option<bool>,
  pub map_root: Option<String>,
  pub source_root: Option<String>,
  pub no_fallthrough_cases_in_switch: Option<bool>,
  pub no_implicit_returns: Option<bool>,
  pub no_property_access_from_index_signature: Option<bool>,
  pub no_unchecked_indexed_access: Option<bool>,
  pub no_unused_locals: Option<bool>,
  pub emit_decorator_metadata: Option<bool>,
  pub experimental_decorators: Option<bool>,
  pub allow_unreachable_code: Option<bool>,
  pub allow_unused_labels: Option<bool>,
  pub assume_changes_only_affect_direct_dependencies: Option<bool>,
  #[deprecated]
  pub charset: Option<String>,
  pub declaration_dir: Option<String>,
  #[deprecated]
  pub diagnostics: Option<bool>,
  pub disable_referenced_project_load: Option<bool>,
  pub disable_size_limit: Option<bool>,
  pub disable_solution_searching: Option<bool>,
  pub disable_source_of_project_reference_redirect: Option<bool>,
  #[serde(rename = "emitBOM")]
  pub emit_bom: Option<bool>,
  pub emit_declaration_only: Option<bool>,
  pub explain_files: Option<bool>,
  pub extended_diagnostics: Option<bool>,
  pub force_consistent_casing_in_file_names: Option<bool>,
  // XXX: Is generateCpuProfile available from tsconfig? Or just the CLI?
  pub generate_cpu_profile: Option<bool>,

  pub imports_not_used_as_values: Option<String>,
  pub jsx_factory: Option<String>,
  pub jsx_fragment_factory: Option<String>,
  pub jsx_import_source: Option<String>,

  pub keyof_strings_only: Option<bool>,
  pub list_emitted_files: Option<bool>,
  pub list_files: Option<bool>,
  pub max_node_module_js_depth: Option<u32>,
  pub no_emit_helpers: Option<bool>,
  pub no_emit_on_error: Option<bool>,
  pub no_error_truncation: Option<bool>,
  pub no_implicit_use_strict: Option<bool>,
  pub no_lib: Option<bool>,
  pub no_resolve: Option<bool>,
  pub no_strict_generic_checks: Option<bool>,
  #[deprecated]
  pub out: Option<String>,
  pub preserve_const_enums: Option<bool>,
  pub react_namespace: Option<String>,
  pub resolve_json_module: Option<bool>,
  pub skip_default_lib_check: Option<bool>,
  pub skip_lib_check: Option<bool>,
  pub strip_internal: Option<bool>,
  pub suppress_excess_property_errors: Option<bool>,
  pub suppress_implicit_any_index_errors: Option<bool>,
  pub trace_resolution: Option<bool>,
  pub use_define_for_class_fields: Option<bool>,
  pub preserve_watch_output: Option<bool>,
  pub pretty: Option<bool>,
  pub fallback_polling: Option<String>,
  pub watch_directory: Option<String>,
  pub watch_file: Option<String>,
}

/// Module resolution mode
///
/// Specify the module resolution strategy: 'node' (Node.js) or 'classic' (used in TypeScript before the release of 1.6). You probably won’t need to use classic in modern code.
/// There is a handbook reference page [on Module Resolution](https://www.typescriptlang.org/docs/handbook/module-resolution.html).
#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ModuleResolutionMode {
  #[serde(rename = "classic", alias = "Classic")]
  Classic,
  #[serde(rename = "node", alias = "Node", alias = "node10", alias = "Node10")]
  Node,
  #[serde(rename = "node16", alias = "Node16")]
  Node16,
  #[serde(rename = "nodenext", alias = "NodeNext")]
  NodeNext,
  #[serde(rename = "bundler", alias = "Bundler")]
  Bundler,
}

/// Controls how JSX constructs are emitted in JavaScript files. This only affects output of JS files that started in .tsx files.
///
///
/// For example, this sample code:
///
/// ```tsx
/// export const helloWorld = () => <h1>Hello world</h1>;
/// ```
///
/// Default: "react"
///
/// ```tsx
/// export const helloWorld = () => React.createElement("h1", null, "Hello world");
/// ```
///
/// Preserve: "preserve"
///
/// ```tsx
/// export const helloWorld = () => <h1>Hello world</h1>;
/// ```
///
/// React Native: "react-native"
///
/// ```tsx
/// export const helloWorld = () => <h1>Hello world</h1>;
/// ```
///
/// React 17 transform: "react-jsx"
///
/// ```tsx
/// import { jsx as _jsx } from "react/jsx-runtime";
/// export const helloWorld = () => _jsx("h1", { children: "Hello world" }, void 0);
/// ```
///
/// React 17 dev transform: "react-jsxdev"
///
/// ```tsx
/// import { jsxDEV as _jsxDEV } from "react/jsx-dev-runtime";
/// const _jsxFileName = "/home/runner/work/TypeScript-Website/TypeScript-Website/packages/typescriptlang-org/index.tsx";
/// export const helloWorld = () => _jsxDEV("h1", { children: "Hello world" }, void 0, false, { fileName: _jsxFileName, lineNumber: 7, columnNumber: 32 }, this);
/// ```
#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Jsx {
  /// Emit .js files with JSX changed to the equivalent React.createElement calls
  React,
  /// Emit .js files with the JSX changed to _jsx calls
  ReactJsx,
  /// Emit .js files with the JSX to _jsx calls
  ReactJsxdev,
  /// Emit .js files with the JSX unchanged
  ReactNative,
  /// Emit .jsx files with the JSX unchanged
  Preserve,
}

/// The transpilation target for the emitted JavaScript.
///
/// Modern browsers support all `ES6` features, so `ES6` is a good choice. You might choose to set a lower target if your code
/// is deployed to older environments, or a higher target if your code is guaranteed to run in newer environments.
///
/// The target setting changes which JS features are downleveled and which are left intact. For example, an arrow
/// function () => this will be turned into an equivalent function expression if target is `ES5` or lower.
///
/// Changing target also changes the default value of lib. You may “mix and match” target and lib settings
/// as desired, but you could just set target for convenience.
///
/// For developer platforms like Node will have a certain baselines for the their target depending on their version.
/// You can find a set of community organized TSConfigs at tsconfig/bases for common platforms and their versions.
///
/// The special `ESNext` value refers to the highest version your version of TypeScript supports. This setting should be
/// used with caution, since it doesn’t mean the same thing between different TypeScript versions and can
/// make upgrades less predictable.
#[derive(Debug, PartialEq, Clone)]
pub enum Target {
  Es3,
  Es5,
  Es2015,
  Es6,
  Es2016,
  Es7,
  Es2017,
  Es2018,
  Es2019,
  Es2020,
  EsNext,
  Other(String),
}
impl<'de> Deserialize<'de> for Target {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let s = s.to_uppercase();

    let d = match s.as_str() {
      "ES5" => Target::Es5,
      "ES2015" => Target::Es2015,
      "ES6" => Target::Es6,
      "ES2016" => Target::Es2016,
      "ES7" => Target::Es7,
      "ES2017" => Target::Es2017,
      "ES2018" => Target::Es2018,
      "ES2019" => Target::Es2019,
      "ES2020" => Target::Es2020,
      "ESNEXT" => Target::EsNext,
      other => Target::Other(other.to_string()),
    };

    Ok(d)
  }
}

/// Available definitions for built-in JS APIs.
///
/// TypeScript includes a default set of type definitions for built-in JS APIs (like Math), as well as type definitions for things found in browser environments (like document). TypeScript also includes APIs for newer JS features matching the target you specify; for example the definition for Map is available if target is ES6 or newer.
///
/// You may want to change these for a few reasons:
///
/// * Your program doesn't run in a browser, so you don’t want the "dom" type definitions
/// * Your runtime platform provides certain JavaScript API objects (maybe through polyfills), but doesn't yet support the full syntax of a given ECMAScript version
/// * You have polyfills or native implementations for some, but not all, of a higher level ECMAScript version
///
#[derive(Debug, PartialEq, Clone)]
pub enum Lib {
  Es5,
  Es2015,
  Es6,
  Es2016,
  Es7,
  Es2017,
  Es2018,
  Es2019,
  Es2020,
  EsNext,
  Dom,
  WebWorker,
  ScriptHost,
  DomIterable,
  Es2015Core,
  Es2015Generator,
  Es2015Iterable,
  Es2015Promise,
  Es2015Proxy,
  Es2015Reflect,
  Es2015Symbol,
  Es2015SymbolWellKnown,
  Es2016ArrayInclude,
  Es2017Object,
  Es2017Intl,
  Es2017SharedMemory,
  Es2017String,
  Es2017TypedArrays,
  Es2018Intl,
  Es2018Promise,
  Es2018RegExp,
  Es2019Array,
  Es2019Object,
  Es2019String,
  Es2019Symbol,
  Es2020String,
  Es2020SymbolWellknown,
  EsNextAsyncIterable,
  EsNextArray,
  EsNextIntl,
  EsNextSymbol,
  Other(String),
}

impl<'de> Deserialize<'de> for Lib {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let s = s.to_uppercase();

    let d = match s.as_str() {
      "ES5" => Lib::Es5,
      "ES2015" => Lib::Es2015,
      "ES6" => Lib::Es6,
      "ES2016" => Lib::Es2016,
      "ES7" => Lib::Es7,
      "ES2017" => Lib::Es2017,
      "ES2018" => Lib::Es2018,
      "ES2019" => Lib::Es2019,
      "ES2020" => Lib::Es2020,
      "ESNext" => Lib::EsNext,
      "DOM" => Lib::Dom,
      "WEBWORKER" => Lib::WebWorker,
      "SCRIPTHOST" => Lib::ScriptHost,
      "DOM.ITERABLE" => Lib::DomIterable,
      "ES2015.CORE" => Lib::Es2015Core,
      "ES2015.GENERATOR" => Lib::Es2015Generator,
      "ES2015.ITERABLE" => Lib::Es2015Iterable,
      "ES2015.PROMISE" => Lib::Es2015Promise,
      "ES2015.PROXY" => Lib::Es2015Proxy,
      "ES2015.REFLECT" => Lib::Es2015Reflect,
      "ES2015.SYMBOL" => Lib::Es2015Symbol,
      "ES2015.SYMBOL.WELLKNOWN" => Lib::Es2015SymbolWellKnown,
      "ES2015.ARRAY.INCLUDE" => Lib::Es2016ArrayInclude,
      "ES2015.OBJECT" => Lib::Es2017Object,
      "ES2017INTL" => Lib::Es2017Intl,
      "ES2015.SHAREDMEMORY" => Lib::Es2017SharedMemory,
      "ES2017.STRING" => Lib::Es2017String,
      "ES2017.TYPEDARRAYS" => Lib::Es2017TypedArrays,
      "ES2018.INTL" => Lib::Es2018Intl,
      "ES2018.PROMISE" => Lib::Es2018Promise,
      "ES2018.REGEXP" => Lib::Es2018RegExp,
      "ES2019.ARRAY" => Lib::Es2019Array,
      "ES2019.OBJECT" => Lib::Es2019Object,
      "ES2019.STRING" => Lib::Es2019String,
      "ES2019.SYMBOL" => Lib::Es2019Symbol,
      "ES2020.STRING" => Lib::Es2020String,
      "ES2020.SYMBOL.WELLKNOWN" => Lib::Es2020SymbolWellknown,
      "ESNEXT.ASYNCITERABLE" => Lib::EsNextAsyncIterable,
      "ESNEXT.ARRAY" => Lib::EsNextArray,
      "ESNEXT.INTL" => Lib::EsNextIntl,
      "ESNEXT.SYMBOL" => Lib::EsNextSymbol,
      other => Lib::Other(other.to_string()),
    };

    Ok(d)
  }
}

/// Sets the module system for the program.
///
/// See the [Modules reference page](https://www.typescriptlang.org/docs/handbook/modules.html)
/// for more information. You very likely want "CommonJS" for node projects.
///
/// Changing module affects moduleResolution which also has a reference page.
///
/// Here’s some example output for this file:
///
/// ```tsx
/// // @filename: index.ts
/// import { valueOfPi } from "./constants";
///
///
/// export const twoPi = valueOfPi * 2;
/// ```
///
/// ## CommonJS
///
/// ```js
/// "use strict";
/// Object.defineProperty(exports, "__esModule", { value: true });
/// exports.twoPi = void 0;
/// const constants_1 = require("./constants");
/// exports.twoPi = constants_1.valueOfPi * 2;
/// ```
///
/// ## UMD
///
/// ```js
/// (function (factory) {
///     if (typeof module === "object" && typeof module.exports === "object") {
///         var v = factory(require, exports);
///         if (v !== undefined) module.exports = v;
///     }
///     else if (typeof define === "function" && define.amd) {
///         define(["require", "exports", "./constants"], factory);
///     }
/// })(function (require, exports) {
///     "use strict";
///     Object.defineProperty(exports, "__esModule", { value: true });
///     exports.twoPi = void 0;
///     const constants_1 = require("./constants");
///     exports.twoPi = constants_1.valueOfPi * 2;
/// });
/// ```
///
/// ## AMD
///
/// ```js
/// define(["require", "exports", "./constants"], function (require, exports, constants_1) {
///     "use strict";
///     Object.defineProperty(exports, "__esModule", { value: true });
///     exports.twoPi = void 0;
///     exports.twoPi = constants_1.valueOfPi * 2;
/// });
/// ```
///
/// ## System
///
/// ```js
/// System.register(["./constants"], function (exports_1, context_1) {
///     "use strict";
///     var constants_1, twoPi;
///     var __moduleName = context_1 && context_1.id;
///     return {
///         setters: [
///             function (constants_1_1) {
///                 constants_1 = constants_1_1;
///             }
///         ],
///         execute: function () {
///             exports_1("twoPi", twoPi = constants_1.valueOfPi * 2);
///         }
///     };
/// });
/// ```
///
/// ## ESNext
///
/// ```js
/// import { valueOfPi } from "./constants";
/// export const twoPi = valueOfPi * 2;
/// ```
///
/// ## ES2020
///
/// ```js
/// import { valueOfPi } from "./constants";
/// export const twoPi = valueOfPi * 2;
/// ```
///
/// ## None
///
/// ```js
/// "use strict";
/// Object.defineProperty(exports, "__esModule", { value: true });
/// exports.twoPi = void 0;
/// const constants_1 = require("./constants");
/// exports.twoPi = constants_1.valueOfPi * 2;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Module {
  CommonJs,
  Es6,
  Es2015,
  Es2020,
  None,
  Umd,
  Amd,
  System,
  EsNext,
  Other(String),
}

impl<'de> Deserialize<'de> for Module {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let s = s.to_uppercase();

    let r = match s.as_str() {
      "COMMONJS" => Module::CommonJs,
      "ESNEXT" => Module::EsNext,
      "ES6" => Module::Es6,
      "ES2015" => Module::Es2015,
      "ES2020" => Module::Es2020,
      "NONE" => Module::None,
      "UMD" => Module::Umd,
      "AMD" => Module::Amd,
      "SYSTEM" => Module::System,
      other => Module::Other(other.to_string()),
    };

    Ok(r)
  }
}
