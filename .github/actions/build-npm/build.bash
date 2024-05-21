set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

if ! [ "$MACH_VERSION" = "" ]; then
  echo "Building with version $MACH_VERSION"
  just build-publish
else
  echo "Building untagged local version"
  just build
fi
