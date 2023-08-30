#!/bin/bash
set -e
cd "`dirname $0`"

mkdir -p out
cargo build --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm out/contract.wasm