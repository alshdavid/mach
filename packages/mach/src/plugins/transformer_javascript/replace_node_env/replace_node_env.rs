use std::collections::HashMap;

use swc_core::ecma::visit::FoldWith;
use swc_core::ecma::ast::Program;

use super::replace_node_env_fold::NodeEnvReplacer;

pub fn replace_node_env(
  program: Program,
  env: &HashMap<String, String>,
) -> Program {
  program.fold_with(&mut NodeEnvReplacer { env })
}
