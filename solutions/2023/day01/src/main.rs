use std::sync::{mpsc, Arc};

use aoc_2023_day01::{concat_digits, digit_parser, digit_word_parser};
use parsely::Parse;
use shared::{read_input, receive_answers, run_part_threaded};

type Answer = u32;

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input(2023, 1);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn part1(input: &str) -> Answer {
    input
        .lines()
        .map(|line| {
            let (digits, _): (Vec<u32>, _) =
                digit_parser().many(1..).parse(line).expect("parse input");

            concat_digits(digits[0], digits[digits.len() - 1])
        })
        .sum()
}

fn part2(input: &str) -> Answer {
    input
        .lines()
        .map(|line| {
            let (digits, _): (Vec<u32>, _) = digit_word_parser()
                .many(1..)
                .parse(line)
                .expect("parse input");

            concat_digits(digits[0], digits[digits.len() - 1])
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const PART2_INPUT: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_concat_digits() {
        assert_eq!(concat_digits(1, 1), 11);
        assert_eq!(concat_digits(0, 1), 1);
        assert_eq!(concat_digits(1, 0), 10);
        assert_eq!(concat_digits(2, 3), 23);
        assert_eq!(concat_digits(8, 9), 89);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART2_INPUT), 281);
    }

    #[test]
    fn test_part2_edge_cases() {
        assert_eq!(part2("eightwo"), 82);

        assert_eq!(part2("eightwoneight"), 88);
        assert_eq!(part2("eightwoneight\neightwoneight"), 88 + 88);

        assert_eq!(
            part2("klklklnineeeeesevenoneenlklklklesenvnsevonoesnvoonsevnvesnoovneonejsjsj"),
            91
        );
        assert_eq!(part2("oooneight2threeight4"), 14);
    }
}
