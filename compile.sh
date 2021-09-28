#!/bin/sh

# compile wasm release
cargo build --release --target wasm32-unknown-unknown

# we don't care what the user says, we just want the script to give the user
# a chance to abort
echo "copy .wasm???"
read junk

# copy
cp target/wasm32-unknown-unknown/release/balistics.wasm pages/balistics.wasm