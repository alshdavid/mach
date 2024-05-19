if ("${env:mach_version}" -ne "") {
  $env:MACH_VERSION = "${env:mach_version}"
  just build-publish
} else {
  just build
}
