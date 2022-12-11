mod monkey;

use std::sync::{mpsc, Arc};

use monkey::{parse_monkey, Monkey};

use shared::{receive_answers, run_part_threaded};

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = std::fs::read_to_string("./input/2022/day11.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str) -> impl Iterator<Item = Monkey> + '_ {
    input.split("\n\n").map(parse_monkey)
}

fn calculate_monkey_business<F>(mut monkeys: Vec<Monkey>, rounds: usize, worry_reducer: F) -> u64
where
    F: Fn(u64) -> u64,
{
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            {
                while let Some(item) = monkeys[i].items.pop() {
                    let monkey = &mut monkeys[i];

                    let item = (monkey.operation)(item);
                    monkey.inspection_count += 1;
                    let item = worry_reducer(item);

                    let to = if item % monkey.test == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };

                    monkeys[to].items.push(item);
                }
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkeys[0].inspection_count * monkeys[1].inspection_count
}

fn part1(input: &str) -> u64 {
    let monkeys: Vec<Monkey> = parse_input(input).collect();
    calculate_monkey_business(monkeys, 20, |item| item / 3)
}

fn part2(input: &str) -> u64 {
    let monkeys: Vec<Monkey> = parse_input(input).collect();

    // this relies on the fact that all the divisibility tests are for prime numbers
    let lcm: u64 = monkeys.iter().map(|m| m.test).product();

    // item modulo lcm will still have the same result for *any* monkey's divisibility test
    calculate_monkey_business(monkeys, 10_000, |item| item % lcm)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parse_input() {
        let monkeys = parse_input(INPUT).collect::<Vec<Monkey>>();
        assert_eq!(monkeys[0].items, vec![79, 98]);
        assert_eq!((monkeys[1].operation)(5), 5 + 6);
        assert_eq!(monkeys[2].test, 13);
        assert_eq!(monkeys[3].if_true, 0);
        assert_eq!(monkeys[3].if_false, 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2713310158);
    }
}
