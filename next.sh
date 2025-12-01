#!/bin/bash


source .env

for i in $(seq 1 12)
do
    if [ "$i" -lt 10 ]
    then
        ii="0${i}"
    fi

    dir="day${ii}"
    if [ ! -d "${dir}" ]
    then
        cargo new "${dir}"
        curl "https://adventofcode.com/2025/day/${i}/input" -o input.txt --cookie session="${AOC_SECRET}"
        mv input.txt "${dir}/"
        cp templates.rs "${dir}/src/main.rs"
        break
    fi
done