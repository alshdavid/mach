use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ImportSymbol {
  /// import './foo'
  Unnamed,
  /// import { foo } from './foo'
  Named { sym: String },
  /// import { foo: bar } from './foo'
  Renamed { sym: String, sym_as: String },
  /// import foo from './foo'
  Default { sym_as: String },
  /// import * as foo from './foo'
  Namespace { sym_as: String },
  /// import('./foo')
  Dynamic,
  /// require('./foo')
  Commonjs,
}

impl std::fmt::Debug for ImportSymbol {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      ImportSymbol::Unnamed => write!(f, "Unnamed"),
      ImportSymbol::Named { sym } => write!(f, "Named \"{}\"", sym),
      ImportSymbol::Renamed { sym, sym_as } => write!(f, "Renamed \"{}\" as \"{}\"", sym, sym_as),
      ImportSymbol::Default { sym_as } => write!(f, "Default as \"{}\"", sym_as),
      ImportSymbol::Namespace { sym_as } => write!(f, "Namespace \"{}\"", sym_as),
      ImportSymbol::Dynamic => write!(f, "Dynamic"),
      ImportSymbol::Commonjs => write!(f, "Commonjs"),
    }
  }
}
