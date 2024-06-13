use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ModuleSymbol {
  /// import './foo'
  ImportDirect {
    specifier: String,
  },
  /// import { foo } from './foo'
  ImportNamed {
    sym: String,
    specifier: String,
  },
  /// import { foo: bar } from './foo'
  ImportRenamed {
    sym: String,
    sym_as: String,
    specifier: String,
  },
  /// import foo from './foo'
  ImportDefault {
    sym_as: String,
    specifier: String,
  },
  /// import * as foo from './foo'
  ImportNamespace {
    sym_as: String,
    specifier: String,
  },
  /// import('./foo')
  ImportDynamic {
    specifier: String,
  },
  /// const { foo } = await import('./foo')
  ImportDynamicNamed {
    specifier: String,
    sym: String,
  },
  /// const { foo: foo_renamed } = await import('./foo')
  ImportDynamicRenamed {
    specifier: String,
    sym: String,
    sym_as: String,
  },
  // const bar = ''
  // export { bar }
  ExportNamed {
    sym: String,
  },
  // const foobar = { bar }
  // export const { bar } = foobar
  ExportDestructured {
    sym: String,
    sym_source: String,
  },
  // const foobar = { bar }
  // export const { bar: foo } = foobar
  ExportDestructuredRenamed {
    sym: String,
    sym_as: String,
    sym_source: String,
  },
  // export const foo = ''
  // export { foo as bar }
  ExportRenamed {
    sym: String,
    sym_as: String,
  },
  // export default foo
  ExportDefault,
  // export * from './foo'
  ReexportAll {
    specifier: String,
  },
  // export { foo } from './foo'
  ReexportNamed {
    sym: String,
    specifier: String,
  },
  // export { foo as bar } from './foo'
  ReexportRenamed {
    sym: String,
    sym_as: String,
    specifier: String,
  },
  // export * as foo from './foo'
  ReexportNamespace {
    sym_as: String,
    specifier: String,
  },
  /// require('./foo')
  ImportCommonjs {
    specifier: String,
  },
  /// const { foo } = require('./foo')
  ImportCommonjsNamed {
    specifier: String,
    sym: String,
  },
  // module.exports.foo = ''
  //
  // const foo = ''
  // module.exports.foo = foo
  //
  // const foo = ''
  // module.exports = { foo }
  ExportCommonjsNamed {
    sym: String,
  },
  // module.exports = ''
  ExportCommonjs,
}
