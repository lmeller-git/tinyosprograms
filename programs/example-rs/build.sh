#! /bin/bash

cargo build --release

if [ ! -L "a.out" ]; then
    ln -s "$(pwd)/target/target/release/example-rs" a.out
fi
