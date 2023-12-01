use std::{
    cmp::Ordering,
    fmt,
    sync::{mpsc, Arc},
};

use serde::{Deserialize, Serialize};
use shared::{read_input, receive_answers, run_part_threaded};

type Answer = usize;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Expr {
    List(Vec<Expr>),
    Int(u32),
}

impl Expr {
    fn with_slice<T>(&self, f: impl FnOnce(&[Expr]) -> T) -> T {
        match self {
            Expr::List(list) => f(list.as_slice()),
            int => f(&[int.clone()]),
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).expect("debug"))
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Expr::Int(a), Expr::Int(b)) => a.cmp(b),
            (a, b) => a.with_slice(|a| {
                b.with_slice(|b| {
                    a.iter()
                        .zip(b.iter())
                        .map(|(a, b)| a.cmp(b))
                        .find(|&ordering| ordering != Ordering::Equal)
                        .unwrap_or_else(|| a.len().cmp(&b.len()))
                })
            }),
        }
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input(2022, 13);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input_part1(input: &str) -> Vec<(Expr, Expr)> {
    input
        .split("\n\n")
        .map(|pair| {
            let mut exprs = pair
                .lines()
                .filter_map(|line| serde_json::from_str(line).ok());
            (exprs.next().expect("input"), exprs.next().expect("input"))
        })
        .collect()
}

fn parse_input_part2(input: &str) -> Vec<Expr> {
    input
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect()
}

fn part1(input: &str) -> Answer {
    let mut sum = 0;
    let mut index = 0;

    for pair in parse_input_part1(input) {
        index += 1;

        if pair.0 < pair.1 {
            sum += index;
        }
    }
    sum
}

fn part2(input: &str) -> Answer {
    let divider_one = Expr::List(vec![Expr::List(vec![Expr::Int(2)])]);
    let divider_two = Expr::List(vec![Expr::List(vec![Expr::Int(6)])]);

    let mut exprs = parse_input_part2(input);
    exprs.push(divider_one.clone());
    exprs.push(divider_two.clone());
    exprs.sort();
    exprs
        .iter()
        .enumerate()
        .map(|(i, expr)| (i + 1, expr))
        .filter(|(_, expr)| **expr == divider_one || **expr == divider_two)
        .map(|(i, _)| i)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_match_divider() {
        assert_eq!(
            serde_json::from_str::<Expr>("[[2]]").unwrap(),
            Expr::List(vec![Expr::List(vec![Expr::Int(2)])])
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 140);
    }
}
