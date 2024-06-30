#!/bin/sh

cargo build --release
available_searches=("treat-search" "breadth-first" "uniform-cost" "depth-limited" "iterative-deepening" "bi-directional")
# fname="data/com-lj.ungraph.txt.gz"
fname="data/email-Enron.txt.gz"

mkdir -p massif
mkdir -p search_output

for searchname in "${available_searches[@]}"
do
  valgrind --tool=massif --massif-out-file="massif/$searchname.massif" ./target/release/eia -F "$fname" > "search_output/$searchname.txt"
done
python ./graph.py $(find "massif/" -type f -print 2>/dev/null)
