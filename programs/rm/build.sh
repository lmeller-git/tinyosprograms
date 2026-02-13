#! /bin/bash

echo "building rm in $(pwd)"

cargo build --release --target target.json -Zjson-target-spec

echo "rm built"
