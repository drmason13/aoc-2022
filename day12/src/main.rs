use std::{
    fmt,
    sync::{mpsc, Arc},
};

use pathfinding::{shortest_path, Neighbours};
use shared::{
    receive_answers, run_part_threaded,
    types_2d::{directions_clockwise, Coords, Direction, Grid, NeighbourIter, Size, Vector},
};

use crate::pathfinding::shortest_path_to_dynamic_goal;

mod pathfinding;

#[derive(Clone)]
struct HeightMap {
    heights: Grid<u8>,
    start: Coords,
    end: Coords,
}

impl HeightMap {
    fn get(&self, coords: Coords) -> Option<u8> {
        self.heights.get(coords).copied()
    }

    fn dimensions(&self) -> Size {
        self.heights.dimensions()
    }

    fn plot_route(&self, path: &Vec<Coords>) {
        let Size { width, height } = self.dimensions();
        let blanks = vec![vec!['.'; width]; height];
        let mut grid = Grid::new(blanks);

        for coords in path {
            if let Some(ch) = grid.get_mut(*coords) {
                *ch = self.get(*coords).map(|n| n as char).unwrap();
            }
        }

        println!("{:?}", grid);
    }
}

impl fmt::Debug for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.heights.cells.iter() {
            for height in row {
                write!(f, "{}", *height as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> Neighbours<'a> for HeightMap {
    type Idx = Coords;
    type Iter = NeighbourIter<'a>;

    fn neighbours(&'a self, coords: Coords) -> Self::Iter {
        let height_neighbours = directions_clockwise(Direction::Up).filter_map(move |dir| {
            let height = self
                .heights
                .get(coords)
                .expect("asked for neighbours of coords that weren't found in the grid");

            let neighbour_coords = (coords + Vector::from(dir)).ok()?;
            let neighbour_height = self.heights.get(neighbour_coords)?;

            if *neighbour_height > height + 1 {
                None
            } else {
                Some(neighbour_coords)
            }
        });
        NeighbourIter::new(height_neighbours)
    }
}

struct ReversePathHeightMap(HeightMap);

impl ReversePathHeightMap {
    fn get(&self, coords: Coords) -> Option<u8> {
        self.0.get(coords)
    }
}

impl<'a> Neighbours<'a> for ReversePathHeightMap {
    type Idx = Coords;
    type Iter = NeighbourIter<'a>;

    fn neighbours(&'a self, coords: Coords) -> Self::Iter {
        let height_neighbours = directions_clockwise(Direction::Up).filter_map(move |dir| {
            let height = self
                .0
                .heights
                .get(coords)
                .expect("asked for neighbours of coords that weren't found in the grid");

            let neighbour_coords = (coords + Vector::from(dir)).ok()?;
            let neighbour_height = self.0.heights.get(neighbour_coords)?;

            if *neighbour_height < height - 1 {
                None
            } else {
                Some(neighbour_coords)
            }
        });
        NeighbourIter::new(height_neighbours)
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = std::fs::read_to_string("./input/2022/day12.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str) -> HeightMap {
    let mut start = Coords::zero();
    let mut end = Coords::zero();

    let heights = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    'S' => {
                        start = Coords { x, y };
                        b'a'
                    }
                    'E' => {
                        end = Coords { x, y };
                        b'z'
                    }
                    ch => ch as u8,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    HeightMap {
        heights: Grid::new(heights),
        start,
        end,
    }
}

fn part1(input: &str) -> usize {
    let height_map = parse_input(input);
    let shortest_path = shortest_path(&height_map, height_map.start, height_map.end);
    height_map.plot_route(&shortest_path);
    shortest_path.len() - 1
}

fn part2(input: &str) -> usize {
    let height_map = ReversePathHeightMap(parse_input(input));

    let shortest_path =
        shortest_path_to_dynamic_goal(&height_map, height_map.0.end, |coords, height_map| {
            height_map.get(coords) == Some(b'a')
        });

    height_map.0.plot_route(&shortest_path);
    shortest_path.len() - 1
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_height_neighbours() {
        let height_map = parse_input(INPUT);
        let mut neighbours = height_map.neighbours(Coords { x: 0, y: 0 });
        assert_eq!(neighbours.next().unwrap(), Coords { x: 1, y: 0 });
        assert_eq!(neighbours.next().unwrap(), Coords { x: 0, y: 1 });
        assert_eq!(neighbours.next(), None);

        let mut neighbours = height_map.neighbours(Coords { x: 3, y: 1 });
        assert_eq!(neighbours.next().unwrap(), Coords { x: 3, y: 0 });
        assert_eq!(neighbours.next().unwrap(), Coords { x: 3, y: 2 });
        assert_eq!(neighbours.next().unwrap(), Coords { x: 2, y: 1 });
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn test_reverse_height_neighbours() {
        let height_map = ReversePathHeightMap(dbg!(parse_input(INPUT)));
        let mut neighbours = height_map.neighbours(Coords { x: 0, y: 0 });
        assert_eq!(neighbours.next().unwrap(), Coords { x: 1, y: 0 });
        assert_eq!(neighbours.next().unwrap(), Coords { x: 0, y: 1 });
        assert_eq!(neighbours.next(), None);

        let mut neighbours = height_map.neighbours(Coords { x: 3, y: 1 });
        assert_eq!(neighbours.next().unwrap(), Coords { x: 3, y: 0 });
        assert_eq!(neighbours.next().unwrap(), Coords { x: 4, y: 1 });
        assert_eq!(neighbours.next().unwrap(), Coords { x: 3, y: 2 });
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 29);
    }
}
