set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

ls -l $ROOT_DIR
cd $ROOT_DIR

pnpm install
          
export BRANCH_NAME=${GITHUB_REF##*/}

export NPM_TAG=$BRANCH_NAME
if [ $NPM_TAG = "main" ]; then
  export NPM_TAG="latest"
fi

NEXT_MACH_VERSION="$(node $ROOT_DIR/.github/scripts/ci/next-npm-version.mjs)"

touch "$ROOT_DIR/.env"

echo "NEXT_MACH_VERSION=$NEXT_MACH_VERSION" >> "$ROOT_DIR/release.env"
echo "BRANCH_NAME=$BRANCH_NAME" >> "$ROOT_DIR/release.env"
echo "NPM_TAG=$NPM_TAG" >> "$ROOT_DIR/release.env"

echo NEXT_MACH_VERSION = $NEXT_MACH_VERSION
echo NPM_TAG = $NPM_TAG
echo BRANCH_NAME = $BRANCH_NAME

cat "$ROOT_DIR/.env"

