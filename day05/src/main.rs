use std::{
    fmt,
    str::FromStr,
    sync::{mpsc, Arc},
};

use once_cell::sync::Lazy;
use regex::Regex;

use shared::{receive_answers, run_part_threaded, ValueError};

struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Stack { crates: Vec::new() }
    }

    fn top(&self) -> Option<char> {
        self.crates.last().copied()
    }

    fn lift(&mut self, count: usize) -> Stack {
        let (bottom, top) = self.crates.split_at(self.crates.len() - count);
        let lifted = top.to_vec();
        self.crates = bottom.to_vec();
        Stack { crates: lifted }
    }

    fn place(&mut self, stack: Stack) {
        self.crates.extend(stack.crates);
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ch in self.crates.iter() {
            write!(f, "{ch}")?;
        }
        Ok(())
    }
}

// note that Vec is 0 indexed but instructions are 1 indexed
// we'll handle this entirely at parse time, converting the input
// into 0-indexed usizes suitable for addressing the stacks Vec
struct Instruction {
    /// number of crates to move
    count: usize,
    /// index of the stack to move crates from
    from: usize,
    /// index of the stack to move crates to
    to: usize,
}

impl Instruction {
    fn expect_parse(s: &str) -> Self {
        s.parse().expect("valid input")
    }
}

static INSTRUCTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").expect("valid regex")
});

impl FromStr for Instruction {
    type Err = ValueError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = INSTRUCTION_REGEX
            .captures(s)
            .ok_or_else(|| ValueError(s.to_string()))?;

        let count: usize = captures
            .name("count")
            .ok_or_else(|| ValueError(s.to_string()))?
            .as_str()
            .parse()
            .map_err(|_| ValueError(s.to_string()))?;

        let from: usize = captures
            .name("from")
            .ok_or_else(|| ValueError(s.to_string()))?
            .as_str()
            .parse()
            .map_err(|_| ValueError(s.to_string()))?;

        let to: usize = captures
            .name("to")
            .ok_or_else(|| ValueError(s.to_string()))?
            .as_str()
            .parse()
            .map_err(|_| ValueError(s.to_string()))?;

        Ok(Instruction {
            count,
            from: from - 1,
            to: to - 1,
        })
    }
}

struct PuzzleInput {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = std::fs::read_to_string("./input/2022/day5.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str) -> PuzzleInput {
    let mut parts = input.split("\n\n");
    let stacks = parts.next().expect("valid input");
    let instructions = parts.next().expect("valid input");
    PuzzleInput {
        stacks: parse_stacks(stacks),
        instructions: instructions
            .lines()
            .map(Instruction::expect_parse)
            .collect(),
    }
}

fn parse_stacks(input: &str) -> Vec<Stack> {
    input.lines().rev().fold(Vec::new(), |mut stacks, line| {
        if stacks.is_empty() {
            // 1st (bottom) row is numbered stacks
            for _ in 0..=(line.len() - 3) / 4 {
                stacks.push(Stack::new())
            }
            stacks
        } else {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .for_each(|(stack_index, ch)| {
                    if ch != ' ' {
                        stacks[stack_index].crates.push(ch);
                    }
                });
            stacks
        }
    })
}

fn part1(input: &str) -> String {
    let PuzzleInput {
        instructions,
        mut stacks,
    } = parse_input(input);
    for instruction in instructions {
        for _ in 0..instruction.count {
            if let Some(krate) = stacks[instruction.from].crates.pop() {
                stacks[instruction.to].crates.push(krate);
            }
        }
    }
    stacks.iter().filter_map(|stack| stack.top()).collect()
}

fn part2(input: &str) -> String {
    let PuzzleInput {
        instructions,
        mut stacks,
    } = parse_input(input);
    for instruction in instructions {
        let lifted = stacks[instruction.from].lift(instruction.count);
        stacks[instruction.to].place(lifted);
    }
    stacks.iter().filter_map(|stack| stack.top()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), String::from("CMZ"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), String::from("MCD"));
    }
}
