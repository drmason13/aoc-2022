use std::sync::{mpsc, Arc};

use shared::{read_input, receive_answers, run_part_threaded};

#[derive(Debug)]
pub struct SubSequence {
    pub start: usize,
    pub end: usize,
    pub value: String,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input(2022, 6);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn part1(input: &str) -> usize {
    let subsequence = find_first_subsequence_of_unique_chars(input, 4);
    println!("{subsequence:?}");
    subsequence.end
}

fn part2(input: &str) -> usize {
    let subsequence = find_first_subsequence_of_unique_chars(input, 14);
    println!("{subsequence:?}");
    subsequence.end
}

fn find_first_subsequence_of_unique_chars(input: &str, sequence_length: usize) -> SubSequence {
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
            return SubSequence {
                start,
                end,
                value: String::from_utf8_lossy(&input[start..end]).to_string(),
            };
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
