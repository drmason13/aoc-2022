use std::sync::{
    mpsc::{self, channel},
    Arc,
};

use cpu::{Cpu, Instruction};
use crt::Crt;
use shared::{read_input, receive_answers, run_part_threaded};

mod cpu;
mod crt;

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input(2022, 10);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().filter_map(|line| line.parse().ok())
}

fn part1(input: &str) -> String {
    let instructions = parse_input(input);
    let (tx, rx) = channel();
    let mut cpu = Cpu::new(instructions, tx);

    let calculate_signal_strength = |cycle, value| {
        if (cycle + 20) % 40 == 0 {
            cycle as i64 * value
        } else {
            0
        }
    };
    cpu.add_hook("X", calculate_signal_strength);

    let mut total_signal_strength = 0;

    loop {
        match cpu.step() {
            Some(_) => {
                break;
            }
            None => {
                if let Ok(hook_output) = rx.try_recv() {
                    total_signal_strength += hook_output;
                }
            }
        }
    }

    format!("{total_signal_strength}")
}

fn part2(input: &str) -> String {
    let instructions = parse_input(input);

    let (tx, rx) = channel();
    let mut cpu = Cpu::new(instructions, tx);
    Crt::install_sprite_hook(&mut cpu);

    let mut crt_output = String::from("\n");
    let mut crt = Crt::new(rx, &mut crt_output);

    loop {
        match cpu.step() {
            Some(_) => {
                break;
            }
            None => {
                crt.step();
            }
        }
    }

    crt_output
}

#[cfg(test)]
mod test {
    use super::*;

    const LONGER_INPUT: &str = include_str!("test_input.txt");
    const INPUT: &str = r"noop
addx 3
addx -5";

    #[test]
    fn test_parsing() {
        let instructions = parse_input(INPUT).collect::<Vec<_>>();
        assert_eq!(instructions[0], Instruction::Noop);
        assert_eq!(
            instructions[1],
            Instruction::Add {
                register: "X",
                value: 3
            }
        );
        assert_eq!(
            instructions[2],
            Instruction::Add {
                register: "X",
                value: -5
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(LONGER_INPUT), "13140");
    }

    #[test]
    fn test_part2() {
        let output = part2(LONGER_INPUT);
        println!("{output}");
        assert_eq!(
            output,
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
