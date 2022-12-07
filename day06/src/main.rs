use std::sync::{mpsc, Arc};

use shared::{receive_answers, run_part_threaded};

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = std::fs::read_to_string("./input/2022/day6.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn part1(input: &str) -> usize {
    find_first_subsequence_of_unique_chars(input, 4)
}

fn part2(input: &str) -> usize {
    find_first_subsequence_of_unique_chars(input, 14)
}

fn find_first_subsequence_of_unique_chars(input: &str, sequence_length: usize) -> usize {
    let input = input.as_bytes();
    let max_offset = input.len() - sequence_length;
    // loop through every window of sequence_length chars
    for offset in 0..max_offset {
        // loop through every pair of characters looking for a match
        let mut match_found = false;
        let start = offset;
        let end = offset + sequence_length;
        for i in start..end - 1 {
            for j in i + 1..end {
                if input[i] == input[j] {
                    match_found = true;
                }
            }
        }
        if !match_found {
            return end;
        }
    }
    panic!("expected puzzle solution");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = r"bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT2: &str = r"nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT3: &str = r"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT4: &str = r"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 5);
        assert_eq!(part1(INPUT2), 6);
        assert_eq!(part1(INPUT3), 10);
        assert_eq!(part1(INPUT4), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 23);
        assert_eq!(part2(INPUT2), 23);
        assert_eq!(part2(INPUT3), 29);
        assert_eq!(part2(INPUT4), 26);
    }
}
