use std::{
    ops::Range,
    str::FromStr,
    sync::{mpsc, Arc},
};

use shared::{read_input, receive_answers, run_part_threaded, ValueError};

struct Assignment(Range<usize>);

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.0.start >= other.0.start && self.0.end <= other.0.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.0.end >= other.0.start && other.0.end >= self.0.start
    }
}

impl FromStr for Assignment {
    type Err = ValueError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start = parts
            .next()
            .ok_or_else(|| ValueError(s.into()))?
            .parse()
            .map_err(|_| ValueError(s.into()))?;
        let end = parts
            .next()
            .ok_or_else(|| ValueError(s.into()))?
            .parse()
            .map_err(|_| ValueError(s.into()))?;
        Ok(Assignment(Range { start, end }))
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input(2022, 4);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str) -> Box<dyn Iterator<Item = (Assignment, Assignment)> + '_> {
    Box::new(input.lines().map(|line| {
        let mut parts = line.split(',');
        (
            parts
                .next()
                .expect("missing input")
                .parse()
                .expect("parse input"),
            parts
                .next()
                .expect("missing input")
                .parse()
                .expect("parse input"),
        )
    }))
}

fn part1(input: &str) -> u32 {
    parse_input(input)
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    parse_input(input).filter(|(a, b)| a.overlaps(b)).count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4);
    }

    #[test]
    fn test_parse_assignment() {
        let input = "1-2";
        assert_eq!(
            input.parse::<Assignment>().expect("test").0,
            Range { start: 1, end: 2 }
        );
    }

    #[test]
    fn test_overlaps() {
        let a = "1-4".parse::<Assignment>().expect("test");
        let b = "4-9".parse::<Assignment>().expect("test");
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));

        let a = "1-2".parse::<Assignment>().expect("test");
        let b = "7-9".parse::<Assignment>().expect("test");
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }
}
