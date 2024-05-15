#!/bin/bash
set -ev

JOB_NAME="linux-arm64"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

source $ROOT_DIR/.github/workflows/platform/unix/setup.bash

sudo apt-get update
sudo apt-get install gcc-aarch64-linux-gnu build-essential
rustup target add aarch64-unknown-linux-gnu
aarch64-linux-gnu-gcc --version
export CC=aarch64-linux-gnu-gcc
export MACH_SKIP_POST_INSTALL="true"

profile=release os=linux arch=arm64 just build-publish

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
