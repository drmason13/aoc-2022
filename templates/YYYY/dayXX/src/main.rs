use std::sync::{mpsc, Arc};

use shared::{read_input, receive_answers, run_part_threaded};

type Answer = u32;

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input($YEAR, $DAY);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn part1(input: &str) -> Answer {
    let _ = input;
    todo!("part1");
}

fn part2(input: &str) -> Answer {
    let _ = input;
    todo!("part2");
}

#[cfg(test)]
mod test {
    use shared::indoc;

    use super::*;

    const INPUT: &str = indoc! {r#"
        paste your input here
        the indoc macro takes care of trimming the leading whitespace :)
    "#};

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2);
    }
}
