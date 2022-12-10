use std::{
    fmt,
    iter::FusedIterator,
    sync::{mpsc, Arc},
};

use shared::{
    receive_answers, run_part_threaded,
    types_2d::{iter_coords, Coords, Direction, Size},
};

struct HeightMap {
    heights: Vec<Vec<u32>>,
}

impl HeightMap {
    fn get(&self, coords: Coords) -> Option<u32> {
        self.heights
            .get(coords.y)
            .and_then(|row| row.get(coords.x))
            .copied()
    }

    fn dimensions(&self) -> Size {
        Size {
            width: self.heights[0].len(),
            height: self.heights.len(),
        }
    }
}

impl fmt::Debug for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.heights.iter() {
            for height in row {
                write!(f, "{height}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct HeightMapIter<'a> {
    direction: Direction,
    height_map: &'a HeightMap,
    coords: Coords,
}

struct HeightMapVisibleIter<'a> {
    direction: Direction,
    height_map: &'a HeightMap,
    coords: Coords,
    initial_height: u32,
    // tracks whether we've previously seen a height that blocks our vision
    vision_blocked: bool,
}

impl HeightMap {
    /// iter each height between the height at the given coords and the edge in the given direction
    fn heights_from_point(&self, coords: Coords, direction: Direction) -> HeightMapIter {
        HeightMapIter {
            direction,
            height_map: self,
            coords,
        }
    }

    /// iter each height between the height at the given coords and the edge in the given direction
    /// until (and including!) a height that is >= the starting coords' height.
    fn visible_heights_from_point(
        &self,
        coords: Coords,
        direction: Direction,
    ) -> HeightMapVisibleIter {
        HeightMapVisibleIter {
            direction,
            height_map: self,
            coords,
            initial_height: self.get(coords).expect("initial height"),
            vision_blocked: false,
        }
    }

    /// iter every Coord from left to right and top to bottom
    fn iter_coords(&self) -> impl Iterator<Item = Coords> {
        iter_coords(&self.dimensions())
    }
}

impl<'a> Iterator for HeightMapIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            Direction::Up => {
                self.coords.y = match self.coords.y.checked_sub(1) {
                    Some(n) => n,
                    None => return None,
                }
            }
            Direction::Right => {
                self.coords.x = match self.coords.x.checked_add(1) {
                    Some(n) => n,
                    None => return None,
                }
            }
            Direction::Down => {
                self.coords.y = match self.coords.y.checked_add(1) {
                    Some(n) => n,
                    None => return None,
                }
            }
            Direction::Left => {
                self.coords.x = match self.coords.x.checked_sub(1) {
                    Some(n) => n,
                    None => return None,
                }
            }
        };

        self.height_map.get(self.coords)
    }
}

impl<'a> FusedIterator for HeightMapIter<'a> {}

impl<'a> Iterator for HeightMapVisibleIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vision_blocked {
            return None;
        }
        match self.direction {
            Direction::Up => {
                self.coords.y = match self.coords.y.checked_sub(1) {
                    Some(n) => n,
                    None => return None,
                }
            }
            Direction::Right => {
                self.coords.x = match self.coords.x.checked_add(1) {
                    Some(n) => n,
                    None => return None,
                }
            }
            Direction::Down => {
                self.coords.y = match self.coords.y.checked_add(1) {
                    Some(n) => n,
                    None => return None,
                }
            }
            Direction::Left => {
                self.coords.x = match self.coords.x.checked_sub(1) {
                    Some(n) => n,
                    None => return None,
                }
            }
        };

        if let Some(height) = self.height_map.get(self.coords) {
            if height >= self.initial_height {
                self.vision_blocked = true;
            }
            Some(height)
        } else {
            None
        }
    }
}

impl<'a> FusedIterator for HeightMapVisibleIter<'a> {}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = std::fs::read_to_string("./input/2022/day8.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str) -> HeightMap {
    HeightMap {
        heights: input
            .lines()
            .map(|row| {
                row.chars()
                    .map(|ch| ch.to_digit(10).expect("digit"))
                    .collect()
            })
            .collect(),
    }
}

fn part1(input: &str) -> usize {
    use Direction::*;
    let trees = parse_input(input);
    trees
        .iter_coords()
        .filter(|coords| {
            let this_tree = trees.get(*coords).expect("get tree");
            trees
                .heights_from_point(*coords, Up)
                .all(|height| height < this_tree)
                || trees
                    .heights_from_point(*coords, Right)
                    .all(|height| height < this_tree)
                || trees
                    .heights_from_point(*coords, Down)
                    .all(|height| height < this_tree)
                || trees
                    .heights_from_point(*coords, Left)
                    .all(|height| height < this_tree)
        })
        .count()
}

fn part2(input: &str) -> usize {
    use Direction::*;
    let trees = parse_input(input);
    trees
        .iter_coords()
        .map(|coords| {
            trees.visible_heights_from_point(coords, Up).count()
                * trees.visible_heights_from_point(coords, Right).count()
                * trees.visible_heights_from_point(coords, Down).count()
                * trees.visible_heights_from_point(coords, Left).count()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn test_parser() {
        assert_eq!(format!("{:?}", parse_input(INPUT)).trim(), INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 8);
    }
}
