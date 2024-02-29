// General purpose import
mach_require("module_id", [], (module) => {})

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