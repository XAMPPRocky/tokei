#!/usr/bin/env bash
# Script for downloading cross.
set -e

gh release download --repo cross-rs/cross --pattern "*$2*"
tar -xvzf cross-$1.tar.gz
