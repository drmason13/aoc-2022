use std::sync::{mpsc, Arc};

use itertools::Itertools;
use shared::{receive_answers, run_part_threaded};

struct Rucksack {
    left: Vec<Item>,
    right: Vec<Item>,
}

struct Group(Vec<Item>, Vec<Item>, Vec<Item>);

const A_UPPER_ASCII_CODE: u32 = 65;
const A_LOWER_ASCII_CODE: u32 = 97;

#[derive(PartialEq, Eq)]
struct Item(char);

impl Item {
    fn priority(&self) -> u32 {
        let ascii_code = self.0 as u32;
        if self.0.is_ascii_lowercase() {
            (ascii_code - A_LOWER_ASCII_CODE) + 1
        } else {
            (ascii_code - A_UPPER_ASCII_CODE) + 27
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = std::fs::read_to_string("./input/2022/day3.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input_part1(input: &str) -> Box<dyn Iterator<Item = Rucksack> + '_> {
    Box::new(input.lines().map(|line| {
        let split_index = line.len() / 2;
        let (left, right) = line.split_at(split_index);
        let left = left.chars().map(Item).collect();
        let right = right.chars().map(Item).collect();
        Rucksack { left, right }
    }))
}

fn parse_input_part2(input: &str) -> Box<dyn Iterator<Item = Group> + '_> {
    Box::new(input.lines().tuples().map(|(line1, line2, line3)| {
        Group(
            line1.chars().map(Item).collect(),
            line2.chars().map(Item).collect(),
            line3.chars().map(Item).collect(),
        )
    }))
}

fn part1(input: &str) -> u32 {
    parse_input_part1(input)
        .map(|rucksack| {
            rucksack
                .left
                .iter()
                .find_map(|left| {
                    rucksack
                        .right
                        .iter()
                        .find_map(|right| if left == right { Some(left) } else { None })
                })
                .map(|item| item.priority())
                .unwrap_or(0)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    parse_input_part2(input)
        .map(|group| {
            group
                .0
                .iter()
                .find_map(|item_a| {
                    group.1.iter().find_map(|item_b| {
                        if item_a == item_b {
                            group.2.iter().find_map(|item_c| {
                                if item_b == item_c {
                                    Some(item_c.priority())
                                } else {
                                    None
                                }
                            })
                        } else {
                            None
                        }
                    })
                })
                .unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 70);
    }

    #[test]
    fn priority_lower_works() {
        let item = Item('a');
        assert_eq!(item.priority(), 1);
        let item = Item('b');
        assert_eq!(item.priority(), 2);
        let item = Item('y');
        assert_eq!(item.priority(), 25);
        let item = Item('z');
        assert_eq!(item.priority(), 26);
    }

    #[test]
    fn priority_upper_works() {
        let item = Item('A');
        assert_eq!(item.priority(), 27);
        let item = Item('B');
        assert_eq!(item.priority(), 28);
        let item = Item('Y');
        assert_eq!(item.priority(), 51);
        let item = Item('Z');
        assert_eq!(item.priority(), 52);
    }
}
