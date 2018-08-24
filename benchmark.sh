#!/usr/bin/env bash
set -e

[ "$1" = "--full" ] && FULL=true || FULL=false

echo 'Tokei Benchmarking Tool'

if [ $FULL = true ]; then
    REQUIRED='tokei, cloc, loc, hyperfine, and scc'
else
    REQUIRED='tokei'
fi

echo "The use of this tool requires $REQUIRED to be installed and available in your PATH variable."

echo 'Please enter the path you would like to benchmark:'

read input

hyperfine --version
echo "old tokei: $(tokei --version)"

if [ $FULL = true ]; then
    echo "cloc: $(cloc --version)"
    loc --version
fi

cargo build --release

if [ $FULL = true ]; then
    hyperfine -w 5 "target/release/tokei $input"\
                "tokei $input"\
                "cloc --skip-uniqueness $input"\
                "loc $input"
else
    hyperfine -w 5 "target/release/tokei $input"\
                "tokei $input"\

fi
