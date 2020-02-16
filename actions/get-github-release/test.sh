#!/usr/bin/env bash

set -e

INPUT_REPO=mdbook \
INPUT_MATCHES=apple \
INPUT_OWNER=rust-lang \
INPUT_TOKEN=$GITHUB_API_KEY \
RUNNER_TEMP=./node_modules/temp/runner \
RUNNER_TOOL_CACHE=./node_modules/temp/cache \
node index.js
/tmp/mdbook -h
