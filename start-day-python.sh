#!/bin/bash

mkdir -p $1/python/day$2
cp -n $1/python/template/* $1/python/day$2/
curl https://adventofcode.com/$1/day/$2/input --cookie "session=$(cat session)" > $1/python/day$2/data.txt
cd $1/python/day$2 && uv run --dev ptw . --now -- main.py
