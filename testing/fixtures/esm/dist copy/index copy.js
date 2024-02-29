(async () => {
  const mach_global = (globalThis["PROJECT_HASH"] =
    globalThis["PROJECT_HASH"] || {});
  const mach_init = (mach_global.init = mach_global.init || {});
  const mach_modules = (mach_global.modules = mach_global.modules || {});
  const mach_manifest = (mach_global.manifest = mach_global.manifest || {});

  async function import_script(src) {
    // const existing = document.querySelector(`script[src="${src}"]`);
    // if (existing && existing.loaded) {
    //   return;
    // }
    // const script = existing || document.createElement("script");
    // const onload = new Promise((res, rej) => {
    //   script.onload = res;
    //   script.onerror = rej;
    // });
    // if (!existing) {
    //   script.src = src;
    //   document.head.appendChild(script);
    // }
    // await onload;
  }

  const mach_define_property = (target, key, getter, setter, settings = true) =>
    Object.defineProperty(target, key, {
      get: getter,
      set: setter,
      enumerable: settings,
      configurable: settings,
    });
  const mach_require = (module_id, bundle_ids) => {
    if (mach_modules[module_id]) {
      return mach_modules[module_id];
    }
    const module = {};
    mach_modules[module_id] = module;
    const exports = new Proxy(
      {},
      { get: (_, k) => module[k], set: (_, k, v) => (module[k] = v) }
    );
    mach_define_property(module, "exports", () => exports, undefined, false);
    const define_export = (...args) => mach_define_property(module, ...args);

    const define_reexport = (specifier, bundle_ids, namespace_or_keys) => {
      const compute = (imported) => {
        if (typeof namespace_or_keys === 'undefined') {
          for (let key in imported) mach_define_property(module, key, () => imported[key]);
        } else if (typeof namespace_or_keys === 'string') {
          const target = {}
          for (let key in imported) mach_define_property(target, key, () => imported[key]);
          define_export(namespace_or_keys, () => target)
        } else {
          for (let key of namespace_or_keys) {
            if (typeof key === 'object') {
              define_export(key[1], () => imported[key[0]]);
            } else {
              define_export(key, () => imported[key]);
            }
          }
        }
      }
      const imported = mach_require(specifier, bundle_ids);
      if (imported.then) {
        return imported.then(compute)
      }
      compute(imported)
    };

    const run_init = () => {
      mach_init[module_id](
        mach_require,
        define_export,
        define_reexport,
        module
      );
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
  
  Object.assign(mach_manifest, JSON.parse('{"bfe9874d2390149":"/index.js"}'));

  mach_init["src/e2.js"] = async (
    mach_require,
    define_export,
    define_reexport,
    module
  ) => {
    const e = "e";
    define_export("e", () => e);
    const e1 = "e1";
    define_export("e1", () => e1);
    const e2 = "e2";
    define_export("e2", () => e2);
  };

  mach_init["src/e.js"] = async (
    mach_require,
    define_export,
    define_reexport,
    module
  ) => {
    await define_reexport("src/e2.js", ["bfe9874d2390149"], ["e", "e1", "e2", ["e2", "e3"]]);
    // define_reexport("src/e2.js", undefined, (lazy) => {
    //   define_export("e1", () => lazy.e1);
    //   define_export("e2", () => lazy.e2);
    // });
    // define_reexport("src/e2.js", undefined, (lazy) => {
    //   define_export("e3", () => lazy.e2);
    // });
  };

  mach_init["src/index.js"] = async (
    mach_require,
    define_export,
    define_reexport,
    module
  ) => {
    const ok = await mach_require("src/e.js", ["bfe9874d2390149"]);
    console.log(ok)
  };

  mach_require("src/index.js");
  if (document.currentScript) document.currentScript.loaded = 1;
})();
