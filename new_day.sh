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
CRATE_DIR=solutions/$YEAR/$DAY
TEMPLATE_DIR=templates/YYYY/dayXX

mkdir -p solutions/$YEAR
cd solutions/$YEAR
cargo new $DAY
cd -

cp "$TEMPLATE_DIR/Cargo.toml" "$CRATE_DIR/Cargo.toml"
sed -i "$CRATE_DIR/Cargo.toml" -e "s/\\\$DAY/$DAY/" -e "s/\\\$YEAR/$YEAR/"

cp "$TEMPLATE_DIR/src/main.rs" "$CRATE_DIR/src/main.rs"
sed -i "$CRATE_DIR/src/main.rs" -e "s/\\\$YEAR/$YEAR/" -e "s/\\\$DAY/$2/"

cargo advent --year $YEAR --day $2