#!/bin/bash

dir=$1/python/day$2
mkdir -p $dir
cp -n $1/python/template/* $dir/
curl https://adventofcode.com/$1/day/$2/input --cookie "session=$(cat session)" > $dir/data.txt
uv run --dev ptw $dir --now -- $dir/main.py
