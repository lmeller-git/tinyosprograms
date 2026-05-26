#! /bin/bash

echo "building chmod in $(pwd)"

cargo build --release --target target.json -Zjson-target-spec

echo "chmod built"
