use crate::types::BundleBehavior;
use crate::types::DependencyOptions;
use crate::types::DependencyPriority;
use crate::types::LinkingSymbol;
use crate::types::SpecifierType;

pub fn create_dependencies(linking_symbols: &[LinkingSymbol]) -> Vec<DependencyOptions> {
  let mut dependencies = vec![];

  for linking_symbol in linking_symbols.to_owned().into_iter() {
    let mut dependency = DependencyOptions {
      specifier: Default::default(),
      specifier_type: SpecifierType::ESM,
      priority: DependencyPriority::Sync,
      resolve_from: Default::default(),
      linking_symbol: linking_symbol.clone(),
      bundle_behavior: BundleBehavior::Inline,
    };

    match linking_symbol {
      LinkingSymbol::ImportDirect { specifier } => {
        dependency.specifier = specifier;
      }
      LinkingSymbol::ImportNamed { sym: _, specifier } => {
        dependency.specifier = specifier;
      }
      LinkingSymbol::ImportRenamed {
        sym: _,
        sym_as: _,
        specifier,
      } => {
        dependency.specifier = specifier;
      }
      LinkingSymbol::ImportDefault {
        sym_as: _,
        specifier,
      } => {
        dependency.specifier = specifier;
      }
      LinkingSymbol::ImportNamespace {
        sym_as: _,
        specifier,
      } => {
        dependency.specifier = specifier;
      }
      LinkingSymbol::ImportDynamic { specifier } => {
        dependency.specifier = specifier;
        dependency.priority = DependencyPriority::Lazy;
      }
      LinkingSymbol::ImportDynamicNamed { specifier, sym: _ } => {
        dependency.specifier = specifier;
        dependency.priority = DependencyPriority::Lazy;
      }
      LinkingSymbol::ImportDynamicRenamed {
        specifier,
        sym: _,
        sym_as: _,
      } => {
        dependency.specifier = specifier;
        dependency.priority = DependencyPriority::Lazy;
      }
      LinkingSymbol::ExportNamed { sym: _ } => continue,
      LinkingSymbol::ExportDestructured {
        sym: _,
        sym_source: _,
      } => continue,
      LinkingSymbol::ExportDestructuredRenamed {
        sym: _,
        sym_as: _,
        sym_source: _,
      } => continue,
      LinkingSymbol::ExportRenamed { sym: _, sym_as: _ } => continue,
      LinkingSymbol::ExportDefault => continue,
      LinkingSymbol::ReexportAll { specifier } => dependency.specifier = specifier,
      LinkingSymbol::ReexportNamed { sym: _, specifier } => dependency.specifier = specifier,
      LinkingSymbol::ReexportRenamed {
        sym: _,
        sym_as: _,
        specifier,
      } => dependency.specifier = specifier,
      LinkingSymbol::ReexportNamespace {
        sym_as: _,
        specifier,
      } => dependency.specifier = specifier,
      LinkingSymbol::ImportCommonjs { specifier } => {
        dependency.specifier = specifier;
        dependency.specifier_type = SpecifierType::Commonjs;
      }
      LinkingSymbol::ImportCommonjsNamed { specifier, sym: _ } => {
        dependency.specifier = specifier;
        dependency.specifier_type = SpecifierType::Commonjs;
      }
      LinkingSymbol::ExportCommonjsNamed { sym: _ } => continue,
      LinkingSymbol::ExportCommonjs => continue,
    }

    dependencies.push(dependency);
  }

  dependencies
}
