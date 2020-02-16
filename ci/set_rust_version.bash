#!/usr/bin/env bash
set -e
rustup default $1
rustup target add $2
