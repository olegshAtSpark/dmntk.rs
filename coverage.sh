#!/usr/bin/env bash

cargo clean

export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=abort"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"

cargo test

mkdir ./target/lcov
mkdir ./target/coverage

grcov . --llvm -s . -t lcov --branch --ignore-not-existing --ignore "*cargo*" --ignore "*chrono-tz*" -o ./target/lcov/lcov.info

genhtml -o ./target/coverage ./target/lcov/lcov.info