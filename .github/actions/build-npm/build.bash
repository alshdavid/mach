set -ev

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

source $ROOT_DIR/.github/workflows/platform/unix/setup-env.bash

if ![ "$mach_version" = "" ]; then
  export MACH_VERSION="${mach_version}"
  just build-publish
else
  just build
fi

mkdir $ROOT_DIR/artifacts
cd npm/mach
npm pack
mv *.tgz npm-mach.tgz
mv *.tgz $ROOT_DIR/artifacts/npm-mach.tgz
