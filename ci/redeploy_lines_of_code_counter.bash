#!/usr/bin/env bash
# Script for triggering deploy of lines of code counter.
set -e

gh workflow run deploy.yml --repo exercism/lines-of-code-counter
