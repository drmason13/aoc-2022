use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
    sync::{
        mpsc::{self, channel, Sender},
        Arc,
    },
};

use shared::{receive_answers, run_part_threaded, ValueError};

struct Cpu {
    /// cycle 1 is the first cycle
    cycle: usize,

    registers: HashMap<&'static str, i64>,
    hooks: Vec<Hook>,

    // Anticipating the possibility to add new instructions whilst a Cpu is running
    instructions: VecDeque<Instruction>,

    // an instruction can stick around between cycles - this counts down those cycles and holds the instruction outside of the queue
    queue: Queue,

    // I think it will be possible for hooks to send results calculated by the CPU into a channel for further processing
    hook_output_channel: Sender<i64>,
}

impl Cpu {
    /// Creates a new [`Cpu`].
    fn new(
        instructions: impl Iterator<Item = Instruction>,
        hook_output_channel: Sender<i64>,
    ) -> Self {
        let mut registers = HashMap::new();
        registers.insert("X", 1);

        let instructions: VecDeque<_> = instructions.collect();
        let queue = Queue::new();

        Cpu {
            cycle: 0,
            registers,
            hooks: Vec::new(),
            instructions,
            queue,
            hook_output_channel,
        }
    }

    fn add_hook<F>(&mut self, register: &'static str, routine: F)
    where
        F: Fn(usize, i64) -> i64 + 'static,
    {
        let routine = Box::new(routine);
        self.hooks.push(Hook { register, routine });
    }

    /// The CPU will step through one cycle, completing instructions and triggering hooks
    fn step(&mut self) -> Option<usize> {
        // start a new cycle
        self.cycle += 1;

        // fill queue if it is empty
        if self.queue.is_empty() {
            if let Some(instruction) = self.instructions.pop_front() {
                self.queue.push(instruction);
            } else {
                return Some(self.cycle);
            }
        }

        // hooks operate on CPU state during a cycle - before instructions complete
        self.trigger_hooks();

        // progress queued instructions - an instruction that takes one cycle will now be ready immediately after being loaded
        self.queue.tick();

        // is the instruction in the queue ready?
        if let Some(ready_instruction) = self.queue.ready() {
            self.run_instruction(ready_instruction);
        }

        // we are not finished so don't return a cycle count yet
        None
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        use Instruction::*;
        // println!("executing instruction: {:?}", &instruction);
        match instruction {
            Noop => {}
            Add { register, value } => {
                self.registers.entry(register).and_modify(|e| *e += value);
            }
        }
    }

    fn trigger_hooks(&self) {
        self.hooks.iter().for_each(|hook| {
            let output = (hook.routine)(
                self.cycle,
                self.registers
                    .get(hook.register)
                    .copied()
                    .expect("register"),
            );

            self.hook_output_channel
                .send(output)
                .expect("send hook output channel");
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Noop,
    Add { register: &'static str, value: i64 },
}

impl Instruction {
    fn time_to_complete(&self) -> usize {
        use Instruction::*;
        match self {
            Noop => 1,
            Add {
                register: _,
                value: _,
            } => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = ValueError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        let mut parts = s.split(' ');
        match (parts.next(), parts.next()) {
            (Some("noop"), None) => Ok(Noop),
            (Some(add), Some(value)) if add.starts_with("add") => match add.chars().last() {
                Some('x') => Ok(Add {
                    register: "X",
                    value: value.parse().map_err(|_| ValueError(s.into()))?,
                }),
                _ => Err(ValueError(s.into())),
            },
            _ => Err(ValueError(s.into())),
        }
    }
}

/// A simple ticking queue of (currently one) [`Instruction`].
///
/// When the timer reaches 0 the CPU will remove the instruction from the queue and execute it.
#[derive(Debug)]
struct Queue {
    timer: usize,
    hold: Option<Instruction>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            timer: 0,
            hold: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.hold.is_none()
    }

    fn push(&mut self, instruction: Instruction) {
        self.timer = instruction.time_to_complete();
        self.hold = Some(instruction);
    }

    fn tick(&mut self) {
        self.timer -= 1;
    }

    fn ready(&mut self) -> Option<Instruction> {
        if self.timer == 0 {
            self.hold.take()
        } else {
            None
        }
    }
}

struct Hook {
    /// This hook will read from this register.
    register: &'static str,

    /// This function is called when the hook is triggered.
    ///
    /// It takes the current cycle and value of this hook's register in the CPU.
    routine: Box<dyn Fn(usize, i64) -> i64>,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = std::fs::read_to_string("./input/2022/day10.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().filter_map(|line| line.parse().ok())
}

fn part1(input: &str) -> i64 {
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
            Some(cycle_count) => {
                break println!("CPU finished executing after {} cycles", cycle_count);
            }
            None => {
                if let Ok(hook_output) = rx.try_recv() {
                    total_signal_strength += hook_output;
                }
            }
        }
    }

    total_signal_strength
}

fn part2(_input: &str) -> i64 {
    2
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
    fn test_steps() {
        let instructions = parse_input(INPUT);
        let (tx, _rx) = channel();
        let mut cpu = Cpu::new(instructions, tx);

        let values_mid_cycle = [1, 1, 1, 4, 4];

        // we can test the values during cycles by using a hook
        let hook = move |cycle: usize, value: i64| {
            assert_eq!(value, values_mid_cycle[cycle - 1]);
            0
        };
        // run test hook every cycle
        cpu.add_hook("X", hook);

        cpu.step();
        assert_eq!(cpu.cycle, 1);
        assert_eq!(cpu.registers.get("X"), Some(&1));

        cpu.step();
        assert_eq!(cpu.cycle, 2);
        assert_eq!(cpu.registers.get("X"), Some(&1));

        cpu.step();
        assert_eq!(cpu.cycle, 3);
        assert_eq!(cpu.registers.get("X"), Some(&4));

        cpu.step();
        assert_eq!(cpu.cycle, 4);
        assert_eq!(cpu.registers.get("X"), Some(&4));

        cpu.step();
        assert_eq!(cpu.cycle, 5);
        assert_eq!(cpu.registers.get("X"), Some(&-1));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(LONGER_INPUT), 13140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(LONGER_INPUT), 999);
    }
}
