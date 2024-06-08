/*
  This is a central registry for bundle paths. It helps reduce the number of files
  that are cache invalidated when there are chantges

  example:
    Object.assign(mach_manifest, JSON.parse('{"7ysn3":"/index.js","k2FQA": "/foo.js"}'))
*/
Object.assign(mach_manifest, JSON.parse('MANIFEST_JSON'))
