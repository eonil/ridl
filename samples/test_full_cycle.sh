#!/usr/bin/env sh
function help() {
    echo "run this at repo root"
}
trap help ERR
set -e
test -f Cargo.toml
rm -rf tmp

mkdir -p tmp
cp -R samples/* tmp/

mkdir -p tmp/ridl1
cat tmp/sample.rs | cargo run -- ridl1 > tmp/ridl1/spec.yaml

mkdir -p tmp/openapi3
cat tmp/sample.rs | cargo run -- openapi3 > tmp/openapi3/spec.yaml

mkdir -p tmp/swift5
cat tmp/sample.rs | cargo run -- swift5 > tmp/swift5/main.swift

mkdir -p tmp/typescript4
cat tmp/sample.rs | cargo run -- typescript4 > tmp/typescript4/index.ts
