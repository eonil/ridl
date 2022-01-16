#!/usr/bin/env sh
function help() {
    echo "run this at repo root"
}
trap help ERR
set -e
test -f Cargo.toml
test -f ridl_tests/Cargo.toml

cd ridl_tests
rm -rf tmp
mkdir -p tmp
cp -R tests/images/input/* tmp/
RUN="cargo run --manifest-path ../ridl/Cargo.toml -- "
OPT="--rename camel --skip Salmon"

mkdir -p tmp/ridl1
cat tmp/rust1.rs | $RUN ridl1 $OPT > tmp/ridl1/spec.yaml

mkdir -p tmp/openapi3
cat tmp/rust1.rs | $RUN openapi3 $OPT > tmp/openapi3/spec.yaml

mkdir -p tmp/swift5
cat tmp/rust1.rs | $RUN swift5 $OPT > tmp/swift5/main.swift

mkdir -p tmp/typescript4
cat tmp/rust1.rs | $RUN typescript4 $OPT > tmp/typescript4/index.ts
