#!/usr/bin/env bash

cd "$(dirname "${BASH_SOURCE[0]}")"

cd ../vendor
git submodule update --init

cd ../miner
cargo build --bin storage-miner
