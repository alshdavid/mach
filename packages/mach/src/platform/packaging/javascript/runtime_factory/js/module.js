// "async" is added/removed by the runtime factory depending
// on whether the consumer uses "import()"
mach_init['MODULE_ID'] = async (
  mach_require,
  define_export,
  modules,
  module_id,
) => {
  // module code
}
