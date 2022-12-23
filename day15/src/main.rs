mod sensor;

use std::sync::{mpsc, Arc};

use regex::Regex;
use sensor::{Coverage, Map, Sensor, TotalCoverage};
use shared::{
    receive_answers, run_part_threaded,
    types_2d::{iter_vectors, Vector},
};

use crate::sensor::Entity;

type Answer = usize;

fn parse_input(input: &str) -> Map {
    let mut map = Map::new();
    let sensors = parse_sensors(input);
    let mut top_left = Vector::zero();
    let mut bottom_right = Vector::zero();
    for sensor in sensors {
        let reach = sensor.distance_to_beacon() as isize;
        let left = sensor.position.x - reach;
        let right = sensor.position.x + reach;
        let top = sensor.position.y - reach;
        let bottom = sensor.position.y + reach;

        if left < top_left.x {
            top_left.x = left;
        }
        if right + reach > bottom_right.x {
            bottom_right.x = right;
        }
        if top < top_left.y {
            top_left.y = top;
        }
        if bottom < bottom_right.y {
            bottom_right.y = bottom;
        }

        map.add(sensor.closest_beacon, Entity::Beacon);
        map.add(sensor.position, Entity::Sensor(sensor));
        map.grid.top_left = top_left;
        map.grid.bottom_right = bottom_right;
    }
    map
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    let re = Regex::new(r#"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)"#).unwrap();
    input
        .lines()
        .map(|line| {
            let cap = re.captures(line).expect("valid input");
            let parse_coord = |name| {
                cap.name(name)
                    .expect("valid input")
                    .as_str()
                    .parse::<isize>()
                    .expect("valid input")
            };
            Sensor::new(
                Vector::new(parse_coord("sensor_x"), parse_coord("sensor_y")),
                Vector::new(parse_coord("beacon_x"), parse_coord("beacon_y")),
            )
        })
        .collect()
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = std::fs::read_to_string("./input/2022/day15.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn part1(input: &str) -> Answer {
    part1_inner(input, 2_000_000)
}

fn part1_inner(input: &str, y: isize) -> Answer {
    let map = parse_input(input);
    let sensors: Vec<_> = map.sensors().collect();

    let mut total_coverage = TotalCoverage::new();
    for sensor in sensors.iter() {
        if let Some(sensor_coverage) = sensor.coverage_at_y(y as isize, isize::MIN, isize::MAX) {
            total_coverage.add_coverage(sensor_coverage);
        }
    }

    total_coverage.count()
}

#[allow(unused)]
fn part1_inner_naive(input: &str, y: isize) -> Answer {
    let map = parse_input(input);
    let bounds = map.bounds();

    (bounds.top_left.x..=bounds.bottom_right.x)
        .map(|x| {
            if map.get(x, y) == Some(&Entity::Beacon) {
                return false;
            }
            for sensor in map.sensors() {
                if sensor.distance_to_point(x, y) <= sensor.distance_to_beacon() {
                    // mark as NOT BEACON
                    return true;
                }
            }
            // potential beacon
            false
        })
        .filter(|x| *x)
        .count()
}

fn part2(input: &str) -> Answer {
    part2_inner(input, 4_000_000)
}

fn part2_inner(input: &str, max: isize) -> Answer {
    let map = parse_input(input);
    let sensors: Vec<_> = map.sensors().collect();
    let mut y = 0;
    let mut gap = None;

    while y <= max {
        let mut total_coverage = TotalCoverage::new();
        for sensor in sensors.iter() {
            if let Some(sensor_coverage) = sensor.coverage_at_y(y as isize, 0, max) {
                total_coverage.add_coverage(sensor_coverage);
            }
        }
        if total_coverage.coverage != Some(Coverage::new(0, max)) {
            gap = total_coverage.gap(0, max);
            break;
        }
        y += 1;
    }

    let x = gap.expect("unique valid solution");
    x * 4_000_000 + y as usize
}

// too slow for real inputs
#[allow(unused)]
fn part2_inner_bruteforce(input: &str, max: isize) -> Answer {
    let map = parse_input(input);
    let sensors: Vec<_> = map.sensors().collect();

    let beacon = iter_vectors(Vector::zero(), Vector::new(max, max))
        .find_map(|Vector { x, y }| {
            for sensor in sensors.iter() {
                if sensor.distance_to_point(x, y) <= sensor.distance_to_beacon() {
                    // mark as NOT BEACON
                    return None;
                }
            }
            // potential beacon
            Some(Vector::new(x, y))
        })
        .expect("1 unique puzzle solution");

    ((beacon.x * 4_000_000) + beacon.y) as usize
}

#[cfg(test)]
mod test {
    use crate::sensor::{Coverage, TotalCoverage};

    use super::*;

    const INPUT: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(part1_inner(INPUT, 10), 26);
    }

    #[test]
    fn test_sensor_distance() {
        let mut map = Map::new();
        let sensors = parse_sensors(INPUT);
        for sensor in sensors {
            map.add(sensor.closest_beacon, Entity::Beacon);
            map.add(sensor.position, Entity::Sensor(sensor));
        }

        if let Entity::Sensor(test_sensor) = map.get(8, 7).unwrap() {
            assert_eq!(test_sensor.distance_to_beacon(), 9);
        } else {
            panic!("sensor exists")
        }
    }

    #[test]
    fn test_sensor_coverage() {
        let mut map = Map::new();
        let sensors = parse_sensors(INPUT);
        for sensor in sensors.iter() {
            map.add(sensor.closest_beacon, Entity::Beacon);
            map.add(sensor.position, Entity::Sensor(sensor.clone()));
        }

        let test_row = 10;
        let min = 0;
        let max = 20;

        let total_coverage = TotalCoverage::new();

        if let Entity::Sensor(test_sensor) = map.get(8, 7).unwrap() {
            let test_coverage = test_sensor.coverage_at_y(test_row, min, max);
            assert_eq!(test_coverage.as_ref(), Some(&Coverage::new(2, 14)));
        } else {
            panic!("test sensor");
        }

        let total_coverage = sensors
            .iter()
            .filter_map(|sensor| sensor.coverage_at_y(test_row, min, max))
            .fold(total_coverage, |mut total_coverage, sensor_coverage| {
                total_coverage.add_coverage(sensor_coverage);
                dbg!(total_coverage)
            });

        assert_eq!(total_coverage.coverage, Some(Coverage::new(0, max)));
    }

    #[test]
    fn test_sensors_iter() {
        let mut map = Map::new();
        let sensors = parse_sensors(INPUT);
        let sensor_count = sensors.len();
        for sensor in sensors {
            map.add(sensor.closest_beacon, Entity::Beacon);
            map.add(sensor.position, Entity::Sensor(sensor));
        }

        assert_eq!(map.sensors().count(), sensor_count);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_inner(INPUT, 20), 56000011);
    }

    #[test]
    fn test_part2_bruteforce() {
        assert_eq!(part2_inner_bruteforce(INPUT, 20), 56000011);
    }
}
