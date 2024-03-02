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
  if (mach_modules[module_id]) {
    callback && callback(mach_modules[module_id])
    return mach_modules[module_id];
  }

  mach_modules[module_id] = {};

  const define_export = (...args) => mach_define_property(mach_modules[module_id], ...args);

  const run_init = () => {
    mach_init[module_id](mach_require, define_export, mach_modules, module_id);
    mach_init[module_id] = undefined;
    callback && callback(mach_modules[module_id])
    return mach_modules[module_id];
  };

  if (bundle_ids && bundle_ids.length) {
    return Promise.all(
      bundle_ids.map((bundle_id) => import_script(mach_manifest[bundle_id]))
    ).then(run_init);
  }

  return run_init();
};
