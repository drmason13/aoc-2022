use std::sync::{mpsc, Arc};

use shared::{read_input, receive_answers, run_part_threaded};
use shared::parsing::{self as p, Parse};

type Answer = u64;

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input(2024, 1);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

/// Parses two vertical columns of integers into two lists: the left list and the right list
fn parse_lists(input: &str) -> Result<(Vec<u64>, Vec<u64>), p::ErrorOwned> {
    let (parsed, _remaining) = p::int::<u64>().pad().then(p::int::<u64>().pad()).many(..).parse(input)?;
    Ok(parsed.into_iter().unzip())
}

/// Calculate the total of all the differences between the integers in each list, when each list is sorted in the same order
fn part1(input: &str) -> Answer {
    let (mut first_list, mut second_list) = parse_lists(input).expect("Failed to parse input");
    first_list.sort_unstable();
    second_list.sort_unstable();

    first_list.into_iter().zip(second_list).map(|(a, b)| a.abs_diff(b)).sum()
}

fn count(n: u64, list: &[u64]) -> u64 {
    let count = list.iter().filter(|&&x| x == n).count();
    count as u64
}


/// Calculate a "SimilarityScore" for "the lists" by multiplying each number in the left list by the number of times it appears in the right list.
fn part2(input: &str) -> Answer {
    let (first_list, second_list) = parse_lists(input).expect("Failed to parse input");

    first_list.into_iter().map(|n| n * count(n, &second_list)).sum()
}

#[cfg(test)]
mod test {
    use shared::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "#};

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 31);
    }
}
