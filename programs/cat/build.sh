#! /bin/bash

echo "building cat in $(pwd)"

cargo build --release --target target.json -Zjson-target-spec

echo "cat built"
