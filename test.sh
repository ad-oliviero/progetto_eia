#!/bin/sh

rm massif.out.*
cargo build --release
valgrind --tool=massif ./target/release/eia
python ./graph.py $(find . -name 'massif.out.*' | sort -n | tail -n 1)
