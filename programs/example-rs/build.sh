#! /bin/bash

echo "building example-rs in $(pwd)"

cargo build --release

echo "example-rs built"

if [ ! -e "a.out" ]; then
    if [ ! -f "$(pwd)/target/target/release/example-rs" ]; then
        echo "example-rs binary not found"
        exit 1
    fi
    echo "generating symlink a.out to $(pwd)/target/target/release/example-rs"
    ln -s "$(pwd)/target/target/release/example-rs" a.out
fi
