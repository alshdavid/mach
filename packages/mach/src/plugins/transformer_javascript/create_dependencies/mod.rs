use crate::public::BundleBehavior;
use crate::public::Dependency;
use crate::public::DependencyOptions;
use crate::public::DependencyPriority;
use crate::public::LinkingSymbol;
use crate::public::SpecifierType;

pub fn create_dependencies(linking_symbols: &[LinkingSymbol]) -> Vec<DependencyOptions> {
  let mut dependencies = vec![];

  for linking_symbol in linking_symbols.to_owned().into_iter() {
    let mut dependency = DependencyOptions {
      specifier: Default::default(),
      specifier_type: SpecifierType::ESM,
      priority: DependencyPriority::Sync,
      resolve_from: Default::default(),
      linking_symbol: linking_symbol.clone(),
      bundle_behavior: BundleBehavior::Default,
    };

    match linking_symbol {
      LinkingSymbol::ImportDirect { specifier } => {
        dependency.specifier = specifier;
      }
      LinkingSymbol::ImportNamed { sym, specifier } => {
        dependency.specifier = specifier;
      }
      LinkingSymbol::ImportRenamed {
        sym,
        sym_as,
        specifier,
      } => {
        dependency.specifier = specifier;
      }
      LinkingSymbol::ImportDefault { sym_as, specifier } => {
        dependency.specifier = specifier;
      },
      LinkingSymbol::ImportNamespace { sym_as, specifier } => {
        dependency.specifier = specifier;
      },
      LinkingSymbol::ImportDynamic { specifier } => {
        dependency.specifier = specifier;
        dependency.priority = DependencyPriority::Lazy;
      },
      LinkingSymbol::ImportDynamicNamed { specifier, sym } => {
        dependency.specifier = specifier;
        dependency.priority = DependencyPriority::Lazy;
      },
      LinkingSymbol::ImportDynamicRenamed {
        specifier,
        sym,
        sym_as,
      } => {
        dependency.specifier = specifier;
        dependency.priority = DependencyPriority::Lazy;
      },
      LinkingSymbol::ExportNamed { sym } => {
        continue
      },
      LinkingSymbol::ExportDestructured { sym, sym_source } => {
        continue
      },
      LinkingSymbol::ExportDestructuredRenamed {
        sym,
        sym_as,
        sym_source,
      } => {
        continue
      },
      LinkingSymbol::ExportRenamed { sym, sym_as } => {
        continue
      },
      LinkingSymbol::ExportDefault => {},
      LinkingSymbol::ReexportAll { specifier } => {
        dependency.specifier = specifier
      },
      LinkingSymbol::ReexportNamed { sym, specifier } => {
        dependency.specifier = specifier
      },
      LinkingSymbol::ReexportRenamed {
        sym,
        sym_as,
        specifier,
      } => {
        dependency.specifier = specifier
      },
      LinkingSymbol::ReexportNamespace { sym_as, specifier } => {
        dependency.specifier = specifier
      },
      LinkingSymbol::ImportCommonjs { specifier } => {
        dependency.specifier = specifier;
        dependency.specifier_type = SpecifierType::Commonjs;
      },
      LinkingSymbol::ImportCommonjsNamed { specifier, sym } => {
        dependency.specifier = specifier;
        dependency.specifier_type = SpecifierType::Commonjs;
      },
      LinkingSymbol::ExportCommonjsNamed { sym } => {
        continue
      },
      LinkingSymbol::ExportCommonjs => {
        continue
      },
    }

    dependencies.push(dependency);
  }

  dependencies
}
