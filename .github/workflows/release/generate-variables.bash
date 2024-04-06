set -ev

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

source $ROOT_DIR/.github/workflows/platform/unix/install-nodejs.bash

pnpm install
          
export BRANCH_NAME=${GITHUB_REF##*/}

export NPM_TAG=$BRANCH_NAME
if [ $NPM_TAG = "main" ]; then
  export NPM_TAG="latest"
fi

NEXT_MACH_VERSION="$(node $ROOT_DIR/.github/scripts/ci/next-npm-version.mjs)"

echo "NEXT_MACH_VERSION=$NEXT_MACH_VERSION" >> $GITHUB_OUTPUT
echo "BRANCH_NAME=$BRANCH_NAME" >> $GITHUB_OUTPUT
echo "NPM_TAG=$NPM_TAG" >> $GITHUB_OUTPUT

echo NEXT_MACH_VERSION = $NEXT_MACH_VERSION
echo NPM_TAG = $NPM_TAG
echo BRANCH_NAME = $BRANCH_NAME