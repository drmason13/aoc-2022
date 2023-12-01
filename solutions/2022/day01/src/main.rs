use std::sync::{mpsc, Arc};

use shared::{read_input, receive_answers, run_part_threaded};

type Answer = u32;

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input(2022, 1);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn part1(input: &str) -> Answer {
    let max_calories = input
        .split("\n\n")
        .map(|snacks| {
            snacks
                .lines()
                .map(|calories| calories.parse::<u32>().expect("numerical calories"))
                .sum::<u32>()
        })
        .max();

    max_calories.expect("max calories exist")
}

fn part2(input: &str) -> Answer {
    let mut calories = input
        .split("\n\n")
        .map(|snacks| {
            snacks
                .lines()
                .map(|calories| calories.parse::<u32>().expect("numerical calories"))
                .sum()
        })
        .collect::<Vec<u32>>();

    calories.sort_by(|a, b| b.cmp(a));
    let max_calories = &calories[0..3];

    max_calories.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 45000);
    }
}
