(async () => {
  const mach_global = (globalThis["PROJECT_HASH"] =
    globalThis["PROJECT_HASH"] || {});
  const mach_init = (mach_global.init = mach_global.init || {});
  const mach_modules = (mach_global.modules = mach_global.modules || {});
  const mach_manifest = (mach_global.manifest = mach_global.manifest || {});

  const define_property = (target, key, getter, setter, settings = true) => Object.defineProperty(target, key, {
    get: getter,
    set: setter,
    enumerable: settings,
    configurable: settings,
  });

  /** 
   * @param {string} module_id
   * @param {string[]} [bundle_ids]
   * @returns {Object | Promise<Object>} 
   * */
  const mach_require = (module_id, bundle_ids) => {
    if (mach_modules[module_id]) {
      return mach_modules[module_id];
    }
    
    const module = {};
    const exports = new Proxy({}, { get: (_, k) => module[k], set: (_, k, v) => module[k] = v })
    define_property(module, 'exports', () => exports, undefined, false)
    mach_modules[module_id] = module;
  
    const define_export = (...args) => define_property(module, ...args);
  
    // This can probably be omitted with symbol propagation
    // to trace the origin of an import, optimizing out reexports
    const define_reexport = async (specifier, bundle_ids, namespace) => {
      const lazy = await mach_require(specifier, bundle_ids)
      const target = namespace ? {} : module
      for (let key in lazy) define_property(target, key, () => lazy[key])
      if (namespace) define_export(namespace, () => target)
    };

    const run_init = () => {
      mach_init[module_id](mach_require, define_export, define_reexport, module);
      mach_init[module_id] = undefined;
      return mach_modules[module_id];
    }
  
    if (bundle_ids && bundle_ids.length) {
      return Promise.all(bundle_ids.map(bundle_id => import(mach_manifest[bundle_id])))
        .then(run_init)
    }

    return run_init()
  };
  
  mach_init["src/index.js"] = async (
    mach_require,
    define_export,
    define_reexport,
    module,
  ) => {
    const { ["default"]: foo } = mach_require("src/a.js");
    // console.log(foo);
  };

  mach_init["src/a.js"] = async (
    mach_require,
    define_export,
    define_reexport,
    module
  ) => {
    const cjs = mach_require("src/b.js");
    // console.log(cjs.b)
    define_export("default", () => "a");
  };

  mach_init["src/b.js"] = (
    mach_require,
    define_export,
    define_reexport,
    module,
  ) => {
    const obj = mach_require('src/c.js')
    // console.log(mach_require('src/d.js'))
    // module.exports.a = 'foo'
    console.log(obj.c)
    
    setTimeout(() => {
      console.log(obj.c)
      console.log('up3')
      obj.c = 3
    }, 2000)

    let b = 'foo'
    define_export('b', () => b, v => b = v)
  }

  mach_init["src/c.js"] = (
    mach_require,
    define_export,
    define_reexport,
    module
  ) => {
    // module.exports.b = 'bar'
    let c = 1
    setTimeout(() => {
      console.log('up2')
      c = 2
    }, 1000)

    setTimeout(() => {
      console.log(c)
    }, 3000)

    define_export('c', () => c, v => c = v)
  }

  mach_init["src/d.js"] = async (
    mach_require,
    define_export,
    define_reexport,
    module
  ) => {
    define_export("default", () => "a");
  };

  mach_require("src/index.js");
  if (document.currentScript) document.currentScript.loaded = 1;
})();
