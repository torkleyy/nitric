#!/bin/sh

# Strict script to check the project

cargo fmt --all
RUSTFLAGS="-D warnings" cargo check --all --all-features
RUSTFLAGS="-D warnings" cargo test --all --all-features
RUSTFLAGS="-D warnings" cargo clippy --all --all-features
