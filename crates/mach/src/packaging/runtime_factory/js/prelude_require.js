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
 * @param {(module: *) => void} [callback]
 * @returns {* | Promise<*>}
 * */
const mach_require = (module_id, bundle_ids, callback) => {
  let module = mach_modules[module_id];

  if (module) {
    callback && callback(module)
    return module;
  }

  module = {};
  mach_modules[module_id] = module;

  const define_export = (...args) => mach_define_property(module, ...args);

  const run_init = () => {
    mach_init[module_id](mach_require, define_export, module);
    mach_init[module_id] = undefined;
    callback && callback(module)
    return module;
  };

  if (bundle_ids && bundle_ids.length) {
    return Promise.all(
      bundle_ids.map((bundle_id) => import_script(mach_manifest[bundle_id]))
    ).then(run_init);
  }

  return run_init();
};
