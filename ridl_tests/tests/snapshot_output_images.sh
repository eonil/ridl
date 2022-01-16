#!/usr/bin/env sh
function help() {
    echo "run this at repo root"
}
trap help ERR
set -e
test -f Cargo.toml
test -f ridl_tests/Cargo.toml

function help() {
    echo "last command exit with $?"
}
cd ridl_tests
mkdir -p tests/images/output
RUN="cargo run --manifest-path ../ridl/Cargo.toml -- "

cat tests/images/input/rust1.rs | $RUN ridl1 > tests/images/output/ridl1
cat tests/images/input/rust1.rs | $RUN openapi3 > tests/images/output/openapi3
cat tests/images/input/rust1.rs | $RUN swift5 > tests/images/output/swift5
cat tests/images/input/rust1.rs | $RUN typescript4 > tests/images/output/typescript4
