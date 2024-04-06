set -ev

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

source $ROOT_DIR/.github/workflows/platform/unix/setup.bash

mkdir $ROOT_DIR/artifacts
sudo apt-get update
npm install -g npm pnpm
pnpm install
rustup target add x86_64-unknown-linux-gnu

profile=release os=linux arch=amd64 just build-publish

cd npm/mach
npm pack
mv *.tgz npm-mach.tgz
mv *.tgz $ROOT_DIR/artifacts/npm-mach.tgz
