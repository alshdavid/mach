use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ImportSymbol {
  /// import './foo'
  Unnamed,
  /// import { foo } from './foo'
  /// import { foo: bar } from './foo'
  Named(String),
  /// import foo from './foo'
  Default,
  /// import * as foo from './foo'
  Namespace(String),
  /// export * from './foo'
  Reexport,
  /// import('./foo')
  Dynamic,
  /// require('./foo')
  Commonjs,
}

impl std::fmt::Debug for ImportSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unnamed => write!(f, "Unnamed"),
            Self::Named(arg0) =>  write!(f, "Named \"{}\"", arg0),
            Self::Default => write!(f, "Default"),
            Self::Namespace(arg0) => {
              if arg0 == "" {
                write!(f, "Namespace \"*\"")
              } else {
                write!(f, "Namespace \"{}\"", arg0)
              }
            },
            Self::Reexport => write!(f, "Reexport"),
            Self::Dynamic => write!(f, "Dynamic"),
            Self::Commonjs => write!(f, "Commonjs"),
        }
    }
}
