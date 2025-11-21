#! /bin/bash

echo "building rm in $(pwd)"

cargo build --release

echo "rm built"
