/** @type {Record<string, Function | Object>} */
const __mach_modules = {};
const __mach_default_export = 'default'

/** 
 * @function
 * @param {string} id - The identifier for the module
 * @returns {Object}
 */
function __mach_import_module(id) {
    const module = __mach_modules[id];
    if (!(module && module.constructor && module.call && module.apply)) {
        return module;
    }
    const exports = {};
    __mach_modules[id] = exports;
    module(exports);
    return exports;
}

/** 
 * @function
 * @param {Object} dest - The module object of the current module
 * @param {string} src - The module id of the source module
 * @returns {void}
 */
function __mach_export_all(dest, src){
    const { [__mach_default_export]: _, ...props } = __mach_import_module(src)
    Object.assign(dest, props)
}

/** 
 * @function
 * @param {Object} dest - The module object of the current module
 * @param {*} expr - Thing to export
 * @param {string|number} key - The key to export it as
 * @returns {void}
 */
function __mach_commonjs_export(dest, expr, key = undefined) {
	dest[__mach_default_export] = dest[__mach_default_export] || {};
	if (key !== undefined) {
		dest[key] = expr
		dest[__mach_default_export][key] = expr
	} else {
		Object.assign(dest, expr);
        dest[__mach_default_export] = expr;
	}
	return dest
}