use std::{
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};

type Answer = u32;
type Channel = Sender<Msg<Answer>>;

struct Msg<T>
where
    T: Send,
{
    part: u8,
    value: T,
}

impl<T> Msg<T>
where
    T: Send,
{
    fn new(part: u8, value: T) -> Self {
        Msg { part, value }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = std::fs::read_to_string("./input/2022/day1.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    while let Ok(Msg {
        value: answer,
        part,
    }) = rx.recv()
    {
        println!("Got {} for part {}", answer, part);
    }
}

fn run_part_threaded<F>(part: u8, input: Arc<String>, solver: F, channel: Channel) -> JoinHandle<()>
where
    F: Fn(&str) -> Answer + Send + 'static,
{
    thread::spawn(move || {
        let answer = solver(input.as_ref());
        channel.send(Msg::new(part, answer)).expect("Send answer");
    })
}

fn part1(input: &str) -> u32 {
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

fn part2(input: &str) -> u32 {
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
