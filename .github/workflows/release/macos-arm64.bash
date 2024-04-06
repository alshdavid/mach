#!/bin/bash
set -ev

JOB_NAME="macos-arm64"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

source $ROOT_DIR/.github/workflows/platform/unix/setup.bash

rustup target add aarch64-apple-darwin
pnpm install

profile=release os=macos arch=arm64 just build-publish

mkdir $ROOT_DIR/artifacts
cd $ROOT_DIR/target/$JOB_NAME

mv release mach
tar -czvf mach-$JOB_NAME.tar.gz mach
mv mach-$JOB_NAME.tar.gz $ROOT_DIR/artifacts

cd $ROOT_DIR/npm/mach-os-arch
npm pack
mv *.tgz npm-mach-$JOB_NAME.tgz
mv *.tgz $ROOT_DIR/artifacts/npm-mach-$JOB_NAME.tgz

ls -l $ROOT_DIR/artifacts
