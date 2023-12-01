use std::{
    fmt::Write,
    fs,
    sync::{mpsc, Arc},
};

use regex_lite::{Captures, Regex};
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

fn concat_digits(a: u32, b: u32) -> u32 {
    debug_assert!((0..=9).contains(&a));
    debug_assert!((0..=9).contains(&b));
    a * 10 + b
}

fn first_digit(digits: &str) -> u32 {
    digits.chars().next().unwrap().to_digit(10).unwrap()
}

fn last_digit(digits: &str) -> u32 {
    digits.chars().last().unwrap().to_digit(10).unwrap()
}

fn replace_number_words_with_digits(s: &str) -> String {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();

    re.replace_all(s, |m: &Captures| match &m[0] {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => unreachable!("impossible match with regex"),
    })
    .into_owned()
}

fn part1(input: &str) -> Answer {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();

            match digits.len() {
                0 => panic!("puzzle input must contain digits"),
                _ => concat_digits(first_digit(&digits), last_digit(&digits)),
            }
        })
        .sum()
}

fn part2(input: &str) -> Answer {
    input
        .lines()
        .map(|line| {
            let digits = replace_number_words_with_digits(line)
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();

            println!("{}", &digits);

            match digits.len() {
                0 => panic!("puzzle input must contain digits"),
                _ => concat_digits(first_digit(&digits), last_digit(&digits)),
            }
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
    fn test_replace_number_words_with_digits() {
        assert_eq!(replace_number_words_with_digits("eightwothree"), "8wo3");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART2_INPUT), 281);
    }
}
