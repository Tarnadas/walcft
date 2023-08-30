#!/usr/bin/env bash

# Exit script as soon as a command fails.
set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

NAME="build_ft"

if docker ps -a --format '{{.Names}}' | grep -Eq "^${NAME}\$"; then
    echo "Container exists"
else
docker create \
     --mount type=bind,source=$DIR,target=/host \
     --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
     --name=$NAME \
     -w /host \
     -e RUSTFLAGS='-C link-arg=-s' \
     -it \
     nearprotocol/contract-builder \
     /bin/bash
fi

perl -i -pe 's/\["cdylib", "rlib"\]/\["cdylib"\]/' Cargo.toml

docker start $NAME
docker exec $NAME /bin/bash -c "rustup default 1.69; rustup target add wasm32-unknown-unknown; cargo build -p fungible-token --target wasm32-unknown-unknown --release"

perl -i -pe 's/\["cdylib"\]/\["cdylib", "rlib"\]/' Cargo.toml

mkdir -p out
cp $DIR/target/wasm32-unknown-unknown/release/*.wasm $DIR/out/
