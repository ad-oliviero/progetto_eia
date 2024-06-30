#!/bin/bash

base_url="https://snap.stanford.edu/data"
urls=(
  "$base_url/soc-sign-bitcoinalpha.csv.gz"
  "$base_url/email-Enron.txt.gz"
  "$base_url/roadNet-CA.txt.gz"
  "$base_url/as-skitter.txt.gz"
  "$base_url/cit-Patents.txt.gz"
  "$base_url/bigdata/communities/com-lj.ungraph.txt.gz"
  "$base_url/bigdata/communities/com-youtube.ungraph.txt.gz"
)
mkdir -p data
for url in "${urls[@]}"; do
  file_name=$(printf "%s" "${url##*/}")
  [ ! -f "$file_name" ] && wget -O "data/$file_name" "$url"
done
