#! /bin/bash

echo "building stat in $(pwd)"

cargo build --release --target target.json -Zjson-target-spec

echo "stat built"
