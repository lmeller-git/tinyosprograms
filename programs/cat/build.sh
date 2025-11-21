#! /bin/bash

echo "building cat in $(pwd)"

cargo build --release

echo "cat built"
