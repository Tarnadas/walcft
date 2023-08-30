#!/bin/bash
set -e
cd "`dirname $0`"

mkdir -p out

perl -i -pe 's/\["cdylib", "rlib"\]/\["cdylib"\]/' Cargo.toml

cargo build -p fungible-token --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./out/

perl -i -pe 's/\["cdylib"\]/\["cdylib", "rlib"\]/' Cargo.toml
