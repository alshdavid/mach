use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ExportSymbol {
  // const bar = ''
  // export { bar }
  Named {
    sym: String,
  },
  // const foobar = { bar }
  // export const { bar } = foobar
  Destructured {
    sym: String,
    sym_source: String,
  },
  // const foobar = { bar }
  // export const { bar: foo } = foobar
  DestructuredRenamed {
    sym: String,
    sym_as: String,
    sym_source: String,
  },
  // export const foo = ''
  // export { foo as bar }
  Renamed {
    sym: String,
    sym_as: String,
  },
  // export default foo
  Default,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ReexportSymbol {
  // export * from './foo'
  All,
  // export { foo } from './foo'
  Named { sym: String },
  // export { foo as bar } from './foo'
  Renamed { sym: String, sym_as: String },
  // export * as foo from './foo'
  Namespace { sym_as: String },
}
