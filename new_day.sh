if [[ -z $1 ]]; then
    echo "day number required"
    echo "e.g. bash new_day.sh 7"
    exit 1
fi
DAY='day'$(printf '%02d' "$1")

cargo new $DAY

cp day01/Cargo.toml "$DAY/Cargo.toml"
sed -i "$DAY/Cargo.toml" -e "s/day01/$DAY/"

cp day01/src/main.rs "$DAY/src/main.rs"
sed -i "$DAY/src/main.rs" -e "s/day1/$DAY/"

cargo advent --day $1