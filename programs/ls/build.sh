#! /bin/bash

echo "building ls in $(pwd)"

cargo build --release

echo "ls built"
