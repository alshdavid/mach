$$mach_modules.$ = (id) => {
  const import_func = $$mach_modules.$
  
  const module = $$mach_modules[id];
  if (!(module && module.constructor && module.call && module.apply)) {
      return module;
  }

  const exports = {};

  // Deoptimization for CommonJs
  // Looking at you React...
  function commonjs(expr, key = undefined) {
    exports['default'] = exports['default'] || {};
    if (key !== undefined) {
      exports[key] = expr;
      exports['default'][key] = expr;
    } else {
      exports['default'] = expr;
      if (expr && typeof expr == 'object') {
        for (key in expr) {
          exports[key] = expr[key]
        }
      }
    }
    return exports
  }

  function export_all(src) {
    const { ['default']: _, ...props } = import_func(src);
    Object.assign(exports, props);
  }

  $$mach_modules[id] = exports;
  module(import_func, exports, export_all, commonjs);
  return exports;
}
