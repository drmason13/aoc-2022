mod map;
mod sand;

use std::sync::{mpsc, Arc};

use map::Map;
use sand::FallingSand;
use shared::{receive_answers, run_part_threaded, types_2d::Coords};

type Answer = usize;

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = std::fs::read_to_string("./input/2022/day14.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str, source: Coords) -> Map {
    let rock_seams = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|corner| {
                    let mut coords = corner.split(',');
                    Coords {
                        x: coords.next().unwrap().parse().expect("parse input"),
                        y: coords.next().unwrap().parse().expect("parse input"),
                    }
                })
                .collect()
        })
        .collect::<Vec<_>>();

    Map::new(source, rock_seams)
}

fn part1(input: &str) -> Answer {
    let source = Coords::new(500, 0);
    let mut map = parse_input(input, source);

    // simulate all the falling sand until a grain of sand falls out of bounds
    let mut sand = FallingSand::new(source);
    while let Ok(position) = sand.fall(&map) {
        // the sand will fall until it rests somewhere in the map (or error if it goes out of bounds)
        map.add_sand(position)
            .expect("should not reach as high as source in part1");
        sand = FallingSand::new(source);
    }
    println!("{:?}", map);
    map.count_resting_sand()
}

fn part2(input: &str) -> Answer {
    let source = Coords::new(500, 0);
    let mut map = parse_input(input, source);

    map.add_floor(source);

    let mut sand = FallingSand::new(source);
    while let Ok(position) = sand.fall(&map) {
        match map.add_sand(position) {
            Ok(_) => sand = FallingSand::new(source),
            Err(_) => break,
        }
    }
    println!("{:?}", map);
    map.count_resting_sand() + 1 // for the unit of sand at the source
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_falling_sand() {
        let source = Coords::new(500, 0);
        let mut map = parse_input(INPUT, source);
        let mut sand = FallingSand::new(source);
        let position = sand.fall(&map).expect("test");

        map.add_sand(position)
            .expect("sand should not reach source");
        assert_eq!(position, Coords::new(500, 8));
        assert_eq!(map.count_resting_sand(), 1);

        let mut sand = FallingSand::new(source);
        let position = sand.fall(&map).expect("test");
        map.add_sand(position)
            .expect("sand should not reach source");
        assert_eq!(position, Coords::new(499, 8));
        assert_eq!(map.count_resting_sand(), 2);

        let mut sand = FallingSand::new(source);
        let position = sand.fall(&map).expect("test");
        map.add_sand(position)
            .expect("sand should not reach source");
        assert_eq!(position, Coords::new(501, 8));
        assert_eq!(map.count_resting_sand(), 3);

        let mut sand = FallingSand::new(source);
        let position = sand.fall(&map).expect("test");
        map.add_sand(position)
            .expect("sand should not reach source");
        assert_eq!(position, Coords::new(500, 7));
        assert_eq!(map.count_resting_sand(), 4);

        let mut sand = FallingSand::new(source);
        let position = sand.fall(&map).expect("test");
        map.add_sand(position)
            .expect("sand should not reach source");
        assert_eq!(position, Coords::new(498, 8));
        assert_eq!(map.count_resting_sand(), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 93);
    }
}
