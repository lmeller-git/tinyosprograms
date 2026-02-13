#! /bin/bash

echo "building touch in $(pwd)"

cargo build --release --target target.json -Zjson-target-spec

echo "touch built"
