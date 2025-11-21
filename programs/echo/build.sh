#! /bin/bash

echo "building echo in $(pwd)"

cargo build --release

echo "echo built"
