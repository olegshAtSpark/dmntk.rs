#!/usr/bin/env bash

WORKING_DIRECTORY=$(pwd)
DMNTK_BINARY_PATH="$WORKING_DIRECTORY"/target/debug
MANUAL_TESTS_DIRECTORY="$WORKING_DIRECTORY"/../dmntk.manual.tests

# clean before proceeding
cargo clean

# set instrumenting variables
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=abort"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"

if [ -n "$1" ]; then
  # run tests for specified package
  cargo test -p "$1"
else
  # run all tests
  cargo test
  # run building feel-parser with features, after reformatting, no changes in source code are expected
  cargo build -p dmntk-feel-parser --features=parsing-tables
  cargo fmt -p dmntk-feel-parser
  # build the whole binary before running manual tests
  cargo build
  # run manual tests to take the coverage of the code executed from command-line
  echo "$MANUAL_TESTS_DIRECTORY"
  if [[ -d "$MANUAL_TESTS_DIRECTORY" ]]
  then
    export PATH=$DMNTK_BINARY_PATH:$PATH
    cd "$MANUAL_TESTS_DIRECTORY" || exit 1
    ./run.sh
    cd "$WORKING_DIRECTORY" || exit 1
  fi
fi

# prepare output directories for coverage results
mkdir ./target/lcov
mkdir ./target/coverage
# generate coverage info
grcov . --llvm -s . -t lcov --branch --ignore-not-existing --ignore "*cargo*" --ignore "*chrono-tz*" --ignore "*tests*" -o ./target/lcov/lcov.info
# generate coverage report
genhtml -t "Decision Model and Notation Toolkit" -q -o ./target/coverage ./target/lcov/lcov.info
# display final message
echo ""
echo "open coverage report: file://$WORKING_DIRECTORY/target/coverage/index.html"
echo ""