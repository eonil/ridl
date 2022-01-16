#!/usr/bin/env sh
function help() {
    echo "run this at repo root"
}
trap help ERR
set -e
test -f Cargo.toml

mkdir -p tests/images/output
cat tests/images/input/rust1.rs | cargo run -- ridl1 > tests/images/output/ridl1
cat tests/images/input/rust1.rs | cargo run -- openapi3 > tests/images/output/openapi3
cat tests/images/input/rust1.rs | cargo run -- swift5 > tests/images/output/swift5
cat tests/images/input/rust1.rs | cargo run -- typescript4 > tests/images/output/typescript4
