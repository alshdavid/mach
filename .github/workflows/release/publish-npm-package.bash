set -ev

# SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
# ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

# source $ROOT_DIR/.github/workflows/platform/unix/install-nodejs.bash

# echo "//registry.npmjs.org/:_authToken=${NPM_TOKEN}" >> $HOME/.npmrc

# PACKAGES=(
#   "$ROOT_DIR/artifacts/npm-mach-linux-amd64.tgz"
#   "$ROOT_DIR/artifacts/npm-mach-linux-arm64.tgz"
#   "$ROOT_DIR/artifacts/npm-mach-macos-amd64.tgz"
#   "$ROOT_DIR/artifacts/npm-mach-macos-arm64.tgz"
#   "$ROOT_DIR/artifacts/npm-mach-windows-amd64.tgz"
#   "$ROOT_DIR/artifacts/npm-mach-windows-arm64.tgz"
#   "$ROOT_DIR/artifacts/npm-mach.tgz"
# )

# for PACKAGE in ${PACKAGES[@]}; do
#   if [ "$BRANCH_NAME" == "main" ]; then
#     echo Publishing latest tag
#     npm publish --access=public $PACKAGE
#   else
#     echo Publishing $BRANCH_NAME tag
#     npm publish --access=public --tag="$BRANCH_NAME" $PACKAGE
#   fi
# done