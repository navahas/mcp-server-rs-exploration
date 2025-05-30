#!/usr/bin/env bash

bin=$1

mkdir -p ./bin

build_and_copy() {
    local b=$1
    if [[ ! -f "src/$b.rs" ]]; then
        echo "error: src/$b.rs not found, unable to build bin"
        return
    fi

    cargo build --release --bin "$b" || return
    cp "./target/release/$b" "./bin/$b"
    echo "binary ready in ./bin/$b"
}

if [[ -z $bin ]]; then
    for file in src/*.rs; do
        bname=$(basename "$file" .rs)
        build_and_copy "$bname"
    done
    exit 0
fi

build_and_copy "$bin"
