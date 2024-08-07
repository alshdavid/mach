set -e

if ! [ "$MACH_VERSION" = "" ]; then
  echo "Building with version $MACH_VERSION"
  just build-publish
else
  echo "Building untagged local version"
  just build
fi
