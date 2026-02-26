#! /bin/bash

echo "building inspect-vars in $(pwd)"

cargo build --release --target target.json -Zjson-target-spec

echo "inspect-vars built"
