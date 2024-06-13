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
      linking_symbol: Default::default(),
      bundle_behavior: BundleBehavior::Default,
    };

    match linking_symbol {
      LinkingSymbol::ImportDirect { specifier } => {
        dependency.specifier = specifier;
      }
      LinkingSymbol::ImportNamed { sym, specifier } => {
        dependency.specifier_type = SpecifierType::ESM;
        dependency.specifier = specifier;
      }
      LinkingSymbol::ImportRenamed {
        sym,
        sym_as,
        specifier,
      } => {}
      LinkingSymbol::ImportDefault { sym_as, specifier } => todo!(),
      LinkingSymbol::ImportNamespace { sym_as, specifier } => todo!(),
      LinkingSymbol::ImportDynamic { specifier } => todo!(),
      LinkingSymbol::ImportDynamicNamed { specifier, sym } => todo!(),
      LinkingSymbol::ImportDynamicRenamed {
        specifier,
        sym,
        sym_as,
      } => todo!(),
      LinkingSymbol::ExportNamed { sym } => todo!(),
      LinkingSymbol::ExportDestructured { sym, sym_source } => todo!(),
      LinkingSymbol::ExportDestructuredRenamed {
        sym,
        sym_as,
        sym_source,
      } => todo!(),
      LinkingSymbol::ExportRenamed { sym, sym_as } => todo!(),
      LinkingSymbol::ExportDefault => todo!(),
      LinkingSymbol::ReexportAll { specifier } => todo!(),
      LinkingSymbol::ReexportNamed { sym, specifier } => todo!(),
      LinkingSymbol::ReexportRenamed {
        sym,
        sym_as,
        specifier,
      } => todo!(),
      LinkingSymbol::ReexportNamespace { sym_as, specifier } => todo!(),
      LinkingSymbol::ImportCommonjs { specifier } => todo!(),
      LinkingSymbol::ImportCommonjsNamed { specifier, sym } => todo!(),
      LinkingSymbol::ExportCommonjsNamed { sym } => todo!(),
      LinkingSymbol::ExportCommonjs => todo!(),
    }

    dependencies.push(dependency);
  }

  dependencies
}
