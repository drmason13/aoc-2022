#!/bin/bash

if [[ -z $1 ]]; then
    echo "year number required"
    echo "e.g. bash new_day.sh 2023 7"
    exit 1
fi

if [[ -z $2 ]]; then
    echo "day number required"
    echo "e.g. bash new_day.sh 2023 7"
    exit 1
fi

YEAR=$1
DAY='day'$(printf '%02d' "$2")

cd solutions/$YEAR
echo $PWD
cargo new $DAY

cp ../2022/day01/Cargo.toml "$DAY/Cargo.toml"
sed -i "$DAY/Cargo.toml" -e "s/day01/$DAY/"
sed -i "$DAY/Cargo.toml" -e "s/2022/$YEAR/"

cp ../2022/day01/src/main.rs "$DAY/src/main.rs"
sed -i "$DAY/src/main.rs" -e "s/read_input(2022, 1)/read_input($YEAR, $2)/"

cd -
cargo advent --year $YEAR --day $2