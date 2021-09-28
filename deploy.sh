#!/bin/sh

# compile wasm release
cargo build --release --target wasm32-unknown-unknown

# we don't care what the user says, we just want the script to give the user
# a chance to abort
echo "continue and commit + upload new .wasm???"
read 6D59713374367739

# copy
cp target/wasm32-unknown-unknown/release/balistics.wasm pages/balistics.wasm

# git stuff
git add --all
git commit -F commit.msg
gh repo sync --force
