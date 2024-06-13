use crate::public::BundleBehavior;
use crate::public::Dependency;
use crate::public::DependencyOptions;
use crate::public::DependencyPriority;
use crate::public::ModuleSymbol;
use crate::public::SpecifierType;

pub fn create_dependencies(linking_symbols: &[ModuleSymbol]) -> Vec<DependencyOptions> {
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
      ModuleSymbol::ImportDirect { specifier } => {
        dependency.specifier = specifier;
      }
      ModuleSymbol::ImportNamed { sym, specifier } => {
        dependency.specifier_type = SpecifierType::ESM;
        dependency.specifier = specifier;
      }
      ModuleSymbol::ImportRenamed {
        sym,
        sym_as,
        specifier,
      } => {}
      ModuleSymbol::ImportDefault { sym_as, specifier } => todo!(),
      ModuleSymbol::ImportNamespace { sym_as, specifier } => todo!(),
      ModuleSymbol::ImportDynamic { specifier } => todo!(),
      ModuleSymbol::ImportDynamicNamed { specifier, sym } => todo!(),
      ModuleSymbol::ImportDynamicRenamed {
        specifier,
        sym,
        sym_as,
      } => todo!(),
      ModuleSymbol::ExportNamed { sym } => todo!(),
      ModuleSymbol::ExportDestructured { sym, sym_source } => todo!(),
      ModuleSymbol::ExportDestructuredRenamed {
        sym,
        sym_as,
        sym_source,
      } => todo!(),
      ModuleSymbol::ExportRenamed { sym, sym_as } => todo!(),
      ModuleSymbol::ExportDefault => todo!(),
      ModuleSymbol::ReexportAll { specifier } => todo!(),
      ModuleSymbol::ReexportNamed { sym, specifier } => todo!(),
      ModuleSymbol::ReexportRenamed {
        sym,
        sym_as,
        specifier,
      } => todo!(),
      ModuleSymbol::ReexportNamespace { sym_as, specifier } => todo!(),
      ModuleSymbol::ImportCommonjs { specifier } => todo!(),
      ModuleSymbol::ImportCommonjsNamed { specifier, sym } => todo!(),
      ModuleSymbol::ExportCommonjsNamed { sym } => todo!(),
      ModuleSymbol::ExportCommonjs => todo!(),
    }

    dependencies.push(dependency);
  }

  dependencies
}
