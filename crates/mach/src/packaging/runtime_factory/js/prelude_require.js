/**
 * @param {*} target
 * @param {() => *} [getter]
 * @param {(v: *) => *} [setter]
 * @param {boolean} [settings]
 * @returns {void}
 * */
const mach_define_property = (target, key, getter, setter, settings = true) => Object.defineProperty(target, key, {
  get: getter,
  set: setter,
  enumerable: settings,
  configurable: settings,
});

/**
 * @param {string} module_id
 * @param {string[]} [bundle_ids]
 * @returns {* | Promise<*>}
 * */
const mach_require = (module_id, bundle_ids) => {
  if (mach_modules[module_id]) {
    return mach_modules[module_id];
  }

  const module = {};
  mach_modules[module_id] = module;

  // This is for CJS support and may not be needed depending
  // on how well CJS transformation can be done
  const exports = new Proxy({}, { get: (_, k) => module[k], set: (_, k, v) => (module[k] = v) });
  mach_define_property(module, "exports", () => exports, undefined, false);

  const define_export = (...args) => mach_define_property(module, ...args);

  // This can probably be omitted with symbol propagation
  // to trace the origin of an import, optimizing out reexports
  const define_reexport = async (specifier, bundle_ids, namespace) => {
    const lazy = await mach_require(specifier, bundle_ids);
    const target = namespace ? {} : module;
    for (let key in lazy) mach_define_property(target, key, () => lazy[key]);
    if (namespace) define_export(namespace, () => target);
  };

  const run_init = () => {
    mach_init[module_id](mach_require, define_export, define_reexport, module);
    mach_init[module_id] = undefined;
    return mach_modules[module_id];
  };

  if (bundle_ids && bundle_ids.length) {
    return Promise.all(
      bundle_ids.map((bundle_id) => import_script(mach_manifest[bundle_id]))
    ).then(run_init);
  }

  return run_init();
};
