#!/bin/sh

# compile wasm release
cargo build --release --target wasm32-unknown-unknown

# copy
cp target/wasm32-unknown-unknown/release/balistics.wasm pages/balistics.wasm