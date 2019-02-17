#!/bin/sh

# Strict script to check the project
# pass `+stable` as parameter to use the stable version

cargo +nightly fmt --all # required for .rustfmt.toml
RUSTFLAGS="-D warnings" cargo $1 check --all --all-features
RUSTFLAGS="-D warnings" cargo $1 test --all --all-features
RUSTFLAGS="-D warnings" cargo $1 clippy --all --all-features
