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

  const mach_require = (module_id, bundle_ids, callback) => {
    if (mach_modules[module_id]) {
      callback && callback(mach_modules[module_id]);
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

    const run_init = () => {
      mach_init[module_id](mach_require, define_export, module);
      mach_init[module_id] = undefined;
      callback && callback(mach_modules[module_id]);
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

  mach_init["src/e2.js"] = (mach_require, define_export, module) => {
    const e = "e";
    define_export("e", () => e);
    const e1 = "e1";
    define_export("e1", () => e1);
    const e2 = "e2";
    define_export("e2", () => e2);
  };

  mach_init["src/e.js"] = (mach_require, define_export, module) => {
    // mach_require("module_id", undefined, (module) => {
    //   const target = {};
    //   for (const key in module) Object.defineProperty(target, key, { get: () => module[key] });
    //   define_export("target", () => namespace);
    // });
    // mach_require("src/e2.js", undefined, (module) => {
    //   define_export("e", () => module.e);
    //   define_export("e1", () => module.e1);
    //   define_export("e2", () => module.e2);
    //   define_export("e3", () => module.e2);
    // });

    mach_require("src/e2.js", undefined, (module) => {
      for (const key in module) define_export(key, () => module[key]);
    });

    // mach_require("src/e2.js", undefined, (module) => {
    const target = {};
    for (const key in module)
      Object.defineProperty(target, key, { get: () => module[key] });
    define_export("namespace", () => target);
    // });

    // mach_require("src/e2.js", ["bfe9874d2390149"], ["e", "e1", "e2", ["e2", "e3"]]);
    // define_reexport("src/e2.js", undefined, (lazy) => {
    //   define_export("e1", () => lazy.e1);
    //   define_export("e2", () => lazy.e2);
    // });
    // define_reexport("src/e2.js", undefined, (lazy) => {
    //   define_export("e3", () => lazy.e2);
    // });
  };

  mach_init["src/index.js"] = (mach_require, define_export, module) => {
    const ok = mach_require("src/e.js");
    // const ok = mach_require("src/e.js", ["bfe9874d2390149"]);
    console.log(ok);
  };

  mach_require("src/index.js");
  if (document.currentScript) document.currentScript.loaded = 1;
})();
