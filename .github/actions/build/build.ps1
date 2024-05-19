if (Boolean_expression) {
  $env:MACH_VERSION = "${mach_version}"
  just build-publish
} else {
  just build
}
