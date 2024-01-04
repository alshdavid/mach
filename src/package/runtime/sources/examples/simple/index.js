const $$mach_modules = (globalThis[Symbol.for("mach_modules")] ?? (globalThis[Symbol.for("mach_modules")] = {}))

$$mach_modules.$$import = (id) => {
  const $$import = $$mach_modules.$$import
  
  const module = $$mach_modules[id];
  if (!(module && module.constructor && module.call && module.apply)) {
      return module;
  }

  const exports = {};

  function commonjs(expr, key = undefined) {
    exports['default'] = exports['default'] || {};
    if (key !== undefined) {
      exports[key] = expr;
      exports['default'][key] = expr;
    } else {
      exports['default'] = expr;
    }
  }

  function export_all(src) {
    const { ['default']: _, ...props } = $$import(src);
    Object.assign(exports, props);
  }

  $$mach_modules[id] = exports;
  module($$import, exports, export_all, commonjs);
  return exports;
}

$$mach_modules["d"] = ($$import, $$export)=>{
  const b = 'b';
  $$export["b"] = b;
};

$$mach_modules["c"] = ($$import, $$export)=>{
  const a = 'a';
  $$export["a"] = a;
};

$$mach_modules["b"] = ($$import, $$export, $$export_all, $$export_cjs)=>{
  $$export_all("c")
  $$export_all("d")
};

$$mach_modules["index"] = ($$import, $$export)=>{
    const fromA = $$import("b");
    console.log(fromA);
};

$$mach_modules.$$import("index")
