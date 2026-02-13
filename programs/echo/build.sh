#! /bin/bash

echo "building echo in $(pwd)"

cargo build --release --target target.json -Zjson-target-spec

echo "echo built"
