#!/usr/bin/env bash

# clean before running any test
cargo clean

# set variables
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=abort"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"

# run tests
cargo test

# prepare output directories for coverage results
mkdir ../target/lcov
mkdir ../target/coverage

# generate coverage info
grcov ../. --llvm -s . -t lcov --branch --ignore-not-existing --ignore "*cargo*" --ignore "*chrono-tz*" -o ../target/lcov/lcov.info

# generate coverage report
genhtml -q -o ../target/coverage ../target/lcov/lcov.info

echo ""
echo "open coverage report: file://$(pwd)/../target/coverage/index.html"
echo ""