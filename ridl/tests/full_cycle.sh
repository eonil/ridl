#!/usr/bin/env sh
function help() {
    echo "run this at repo root"
}
trap help ERR
set -e
test -f Cargo.toml
rm -rf tmp

mkdir -p tmp
cp -R tests/images/input/* tmp/

OPT="--rename camel --skip Salmon"

mkdir -p tmp/ridl1
cat tmp/rust1.rs | cargo run -- ridl1 $OPT > tmp/ridl1/spec.yaml

mkdir -p tmp/openapi3
cat tmp/rust1.rs | cargo run -- openapi3 $OPT > tmp/openapi3/spec.yaml

mkdir -p tmp/swift5
cat tmp/rust1.rs | cargo run -- swift5 $OPT > tmp/swift5/main.swift

mkdir -p tmp/typescript4
cat tmp/rust1.rs | cargo run -- typescript4 $OPT > tmp/typescript4/index.ts
