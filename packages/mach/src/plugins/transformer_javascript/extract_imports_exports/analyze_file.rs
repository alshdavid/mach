use swc_core::ecma::ast::*;

use super::walk_cjs::analyze_js_file_cjs;
use super::walk_esm::analyze_js_file_esm;
use crate::public::ModuleSymbol;

pub type AnalyzeFileResult = Vec<ModuleSymbol>;

/// This will look through a JavaScript file and find/label the
/// import/exports within it
pub fn extract_imports_exports(module: &Program) -> AnalyzeFileResult {
  let mut results = analyze_js_file_esm(module);

  if results.len() == 0 {
    results.extend(analyze_js_file_cjs(module))
  }

  return results;
}
