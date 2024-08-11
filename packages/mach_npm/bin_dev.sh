#!/usr/bin/env sh

env NODE_OPTIONS="--conditions="source" --experimental-strip-types --no-warnings" npx mach $@
