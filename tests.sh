#!/bin/sh

echo "Running tests" &&
cargo test --lib &&
cargo test --bin worthItServer --features="buildBin"