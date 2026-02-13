#! /bin/bash

echo "building example-rs in $(pwd)"

cargo build --release --target target.json -Zjson-target-spec

echo "example-rs built"
