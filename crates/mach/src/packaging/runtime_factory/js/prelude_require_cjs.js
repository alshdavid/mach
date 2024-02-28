const mach_require = (module_id, bundle_ids) => {
  const load_bundles =  async (bundle_ids, module_id) => {
    const loading = [];
    for (const bundle_id of bundle_ids) {
      loading.push(import_script(mach_manifest[bundle_id]));
    }
    await Promise.all(loading);
  }

  const define_property = (target, key, getter, setter) =>
    Object.defineProperty(target, key, {
      get: getter,
      set: setter,
      enumerable: true,
      configurable: true,
    });

  const init_module = () => {
    const module = { exports: {} }
    mach_modules[module_id] = module.exports;

    const define_export = (key, getter, setter) =>
      define_property(module.exports, key, getter, setter);

    const define_reexport = async (bundle_ids, specifier, namespace) => {
      const lazy = await mach_require(bundle_ids, specifier);
      const target = namespace ? {} : exports;
      for (let key in lazy) define_property(target, key, () => lazy[key]);
      if (namespace) define_export(namespace, () => target);
    };

    const result = mach_init[module_id](
      mach_require,
      define_export,
      define_reexport,
      module,
      new Proxy({}, { 
        get: (_, k) => module.exports[k],
        set: (_, k, v) => module.exports[k] = v,
      }),
    );

    mach_init[module_id] = undefined;
    
    if (result && result.then) {
      return result.then(() => mach_modules[module_id])
    }

    return mach_modules[module_id];
  }

  if (mach_modules[module_id]) {
    return mach_modules[module_id];
  }

  if (bundle_ids) {
    return load_bundles.then(init_module)
  }
  return init_module()
};