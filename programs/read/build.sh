#! /bin/bash

echo "building read in $(pwd)"

cargo build --release --target target.json -Zjson-target-spec

echo "read built"
