#!/bin/bash

curl https://adventofcode.com/2021/day/$1/input --cookie "session=$(cat session)" > src/bin/day$1/data.txt
