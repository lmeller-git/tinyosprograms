#! /bin/bash

echo "building ls in $(pwd)"

cargo build --release --target target.json -Zjson-target-spec

echo "ls built"
