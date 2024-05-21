set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

mkdir $ROOT_DIR/artifacts
cd npm/mach
npm pack
mv *.tgz npm-mach.tgz
mv *.tgz $ROOT_DIR/artifacts/npm-mach.tgz
