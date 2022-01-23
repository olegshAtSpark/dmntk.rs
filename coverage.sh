#!/usr/bin/env bash

WORKING_DIRECTORY=$(pwd)
DMNTK_BINARY_PATH="$WORKING_DIRECTORY"/target/debug
MANUAL_TESTS_DIRECTORY="$WORKING_DIRECTORY"/../dmntk.manual.tests

# clean before running any test
cargo clean

# set variables
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=abort"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"

# run all tests
cargo test

# run building feel-parser with features, after reformatting, no changes in source code are expected
cargo build -p dmntk-feel-parser --features=parsing-tables
cargo fmt -p dmntk-feel-parser

# build the binary before running manual tests
cargo build

# run manual tests to cover the code executed from command-line
echo "$MANUAL_TESTS_DIRECTORY"
if [[ -d "$MANUAL_TESTS_DIRECTORY" ]]
then
  export PATH=$DMNTK_BINARY_PATH:$PATH
  cd "$MANUAL_TESTS_DIRECTORY" || exit 1
  ./run.sh
  cd "$WORKING_DIRECTORY" || exit 1
fi

# prepare output directories for coverage results
mkdir ./target/lcov
mkdir ./target/coverage

# generate coverage info
grcov . --llvm -s . -t lcov --branch --ignore-not-existing --ignore "*cargo*" --ignore "*chrono-tz*" -o ./target/lcov/lcov.info

# generate coverage report
genhtml -q -o ./target/coverage ./target/lcov/lcov.info

echo ""
echo "open coverage report: file://$WORKING_DIRECTORY/target/coverage/index.html"
echo ""