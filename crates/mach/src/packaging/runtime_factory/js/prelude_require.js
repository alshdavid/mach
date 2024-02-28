const mach_require = async (module_id, bundle_ids) => {
  if (mach_modules[module_id]) {
    return mach_modules[module_id];
  }
  const loading = [];
  if (bundle_ids) for (const bundle_id of bundle_ids) {
    loading.push(import_script(mach_manifest[bundle_id]));
  }
  await Promise.all(loading);
  const module = {};
  mach_modules[module_id] = module;

  const define_property = (target, key, getter, setter) =>
    Object.defineProperty(target, key, {
      get: getter,
      set: setter,
      enumerable: true,
      configurable: true,
    });

  const define_export = (key, getter, setter) => define_property(module, key, getter, setter);

  // This can probably be omitted with symbol propagation
  // to trace the origin of an import, optimizing out reexports
  const define_reexport = async (specifier, bundle_ids, namespace) => {
    const lazy = await mach_require(specifier, bundle_ids)
    const target = namespace ? {} : module
    for (let key in lazy) define_property(target, key, () => lazy[key])
    if (namespace) define_export(namespace, () => target)
  };

  await mach_init[module_id](
    mach_require,
    define_export,
    define_reexport,
  );
  mach_init[module_id] = undefined;
  return mach_modules[module_id];
};
