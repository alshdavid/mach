if ![ "$mach_version" = "" ]; then
  export MACH_VERSION="${mach_version}"
  just build-publish
else
  just build
fi