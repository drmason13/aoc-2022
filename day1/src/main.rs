fn main() {
    let input = std::fs::read_to_string("../input/2022/day1.txt").expect("failed to read input");
    let max_calories = part1(&input);
    println!("{max_calories}");

    let max_calories = part2(&input);
    println!("{max_calories}");
}

fn part1<T: AsRef<str>>(input: T) -> u32 {
    let max_calories = input
        .as_ref()
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

fn part2<T: AsRef<str>>(input: T) -> u32 {
    let mut calories = input
        .as_ref()
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
