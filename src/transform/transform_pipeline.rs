use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::rc::Rc;

use swc_core::common::Globals;
use swc_core::common::Mark;
use swc_core::common::comments::Comments;
use swc_core::common::sync::Lrc;
use swc_core::common::SourceMap;
use swc_core::ecma::visit::FoldWith;
use swc_core::ecma::atoms::JsWord;

use crate::config::AppConfig;
use crate::linking::DependencyIndex;
use crate::public::AssetMap;
use crate::public::DependencyMap;

use super::NodeEnvReplacer;
use super::collect_decls;
use super::transform_js;
use super::transform_jsx;
use super::transform_ts;
use super::transform_tsx;

// TODO Do in parallel
pub fn transform_pipeline(
  _config: &AppConfig,
  asset_map: AssetMap,
  dependency_map: DependencyMap,
  dependency_index: DependencyIndex,
  source_map: Lrc<SourceMap>,
) -> Result<(AssetMap, DependencyMap, DependencyIndex, Lrc<SourceMap>), String> {
  let mut new_asset_map = AssetMap::new();
  let env = get_env();

  for (asset_id, mut asset) in asset_map {
    let source_map = source_map.clone();
    let env = env.clone();
    let mut module = asset.ast;
    let extension = asset.file_path.extension().unwrap();

    asset.ast = swc_core::common::GLOBALS.set(&Globals::new(), move || {
      let _comments: Option<&dyn Comments> = None;
      let _top_level_mark = Mark::fresh(Mark::root());
      let unresolved_mark = Mark::fresh(Mark::root());
      // TODO run dynamic plugins here, passing in the AST

      let decls = collect_decls(&module);
      
      module = module.fold_with(&mut NodeEnvReplacer{
        replace_env: true,
        env: &env,
        is_browser: true,
        decls: &decls,
        used_env: &mut HashSet::new(),
        source_map: &source_map,
        unresolved_mark
      });

      if extension == "js" {
        module = transform_js(source_map.clone(), module);
      } else if extension == "jsx" {
        module = transform_jsx(source_map.clone(), module);
      } else if extension == "ts" {
        module = transform_ts(source_map.clone(), module);
      } else if extension == "tsx" {
        module = transform_tsx(source_map.clone(), module);
      }

      return module;
    });

    new_asset_map.insert(asset_id.clone(), asset);
  }

  return Ok((
    new_asset_map, dependency_map, dependency_index, source_map));
}


fn get_env() -> Rc<HashMap<JsWord, JsWord>> {
  let mut env = HashMap::<JsWord, JsWord>::new();
  for (key, value) in env::vars() {
    env.insert(JsWord::from(key), JsWord::from(value));
  }
  return Rc::new(env);
}