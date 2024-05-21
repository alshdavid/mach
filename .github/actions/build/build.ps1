if ("${env:MACH_VERSION}" -ne "") {
  just build-publish
} else {
  just build
}
