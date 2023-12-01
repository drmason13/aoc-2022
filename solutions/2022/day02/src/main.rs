use std::{
    str::FromStr,
    sync::{mpsc, Arc},
};

use shared::{read_input, receive_answers, run_part_threaded, ValueError};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn value(&self) -> u32 {
        use Choice::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn beats(&self) -> Choice {
        use Choice::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn beaten_by(&self) -> Choice {
        use Choice::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn contest(&self, other: &Choice) -> Outcome {
        use Choice::*;
        use Outcome::*;
        match (self, other) {
            (Rock, Scissors) => Win,
            (Scissors, Paper) => Win,
            (Paper, Rock) => Win,
            (a, b) if a == b => Draw,
            _ => Loss,
        }
    }
}

impl FromStr for Choice {
    type Err = ValueError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Choice::*;
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(ValueError(s.to_string())),
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn value(&self) -> u32 {
        use Outcome::*;
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
}

impl FromStr for Outcome {
    type Err = ValueError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Outcome::*;
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(ValueError(s.to_string())),
        }
    }
}

fn rig_contest(opponent: &Choice, outcome: &Outcome) -> Choice {
    use Outcome::*;
    match outcome {
        Win => opponent.beaten_by(),
        Draw => opponent.clone(),
        Loss => opponent.beats(),
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input(2022, 2);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input_part1(input: &str) -> Box<dyn Iterator<Item = (Choice, Choice)> + '_> {
    Box::new(input.lines().map(|line| {
        let mut choices = line
            .split(' ')
            .map(|letter| Choice::from_str(letter).expect("valid input"));
        (
            choices.next().expect("valid input"),
            choices.next().expect("valid input"),
        )
    }))
}

fn parse_input_part2(input: &str) -> Box<dyn Iterator<Item = (Choice, Outcome)> + '_> {
    Box::new(input.lines().map(|line| {
        let mut codes = line.split(' ');
        (
            Choice::from_str(codes.next().expect("valid input")).expect("valid input"),
            Outcome::from_str(codes.next().expect("valid input")).expect("valid input"),
        )
    }))
}

fn part1(input: &str) -> u32 {
    parse_input_part1(input)
        .map(|(opponent, you)| you.value() + you.contest(&opponent).value())
        .sum()
}

fn part2(input: &str) -> u32 {
    parse_input_part2(input)
        .map(|(opponent, outcome)| {
            let choice = rig_contest(&opponent, &outcome);
            choice.value() + outcome.value()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 12);
    }
}
