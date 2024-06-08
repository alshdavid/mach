// General purpose import
mach_require("module_id", [], (module) => {});

// export * from './specifier'
{
  for (const key in module) define_export(key, () => module[key]);
}

// export * as namespace from './specifier'
{
  const target = {};
  for (const key in module) Object.defineProperty(target, key, { get: () => module[key] });
  // define_export("namespace", () => target); <- added by factory
}

// Accessor for export on const bindings
// accessors for CJS exports
//
// export const foo = ''
// module.exports
// exports
{
  // module.exports.foo = {}
  modules[module_id][key] = target;
  // module.exports = {}
  modules[module_id] = target;
  // module.exports.foo
  modules[module_id][key];
  // module.exports
  modules[module_id];
}
