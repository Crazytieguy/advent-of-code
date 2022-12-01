#!/bin/bash

mkdir -p $1/src/bin/day$2
cp $1/src/bin/template/* $1/src/bin/day$2/
curl https://adventofcode.com/$1/day/$2/input --cookie "session=$(cat session)" > $1/src/bin/day$2/data.txt
