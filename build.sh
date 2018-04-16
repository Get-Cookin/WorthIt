#!/bin/sh

echo "Updating rust" &&
rustup update &&
echo "Updating cargo" &&
cargo update &&
echo "Installing the latest wasm-gc" &&
cargo install --git https://github.com/alexcrichton/wasm-gc --force &&
echo "Building worthItFunctionsLarge.wasm" &&
cargo +stable build --lib --target wasm32-unknown-unknown --release &&
echo "Shrinking worthItFunctionsLarge.wasm with wasm-gc" &&
wasm-gc target/wasm32-unknown-unknown/release/worthItFunctionsLarge.wasm files/worthItFunctions.wasm &&
echo "Building worthItServer with the nightly compiler" &&
env PKG_CONFIG_ALLOW_CROSS=1 cargo +nightly build --bin worthItServer --features="buildBin" --release --target=x86_64-unknown-linux-musl