#! /bin/bash

echo "building touch in $(pwd)"

cargo build --release

echo "touch built"
