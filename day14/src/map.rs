use std::fmt;

use shared::types_2d::{Coords, InfGrid, Itertools, OutOfBounds, Size, Vector};

use crate::sand::Entity;
use Entity::*;

pub struct Map {
    pub bounds: Size,
    grid: InfGrid<Entity>,
}

impl Map {
    pub fn new(source: Coords, rock_seams: Vec<Vec<Coords>>) -> Self {
        let mut map = Map {
            bounds: Size::zero(),
            grid: InfGrid::new_off_center(source),
        };
        for rock_seam in rock_seams {
            map.add_rock_seam(rock_seam);
        }
        map.add_source(source);

        map.bounds = map.grid.bounds();
        map
    }

    pub fn get(&self, coords: Coords) -> Result<Option<Entity>, OutOfBounds> {
        if coords.outside_of(self.bounds, Coords::try_from(self.grid.offset())?) {
            Err(OutOfBounds(Vector::from(coords)))
        } else {
            Ok(self
                .grid
                .get(Vector::from(coords))
                .and_then(|cell| cell.value))
        }
    }

    pub fn add_source(&mut self, position: Coords) {
        self.grid.add(Vector::from(position), Source, false);
    }

    pub fn add_sand(&mut self, position: Coords) {
        self.grid.add(Vector::from(position), Sand, true);
    }

    pub fn add_rock_seam(&mut self, corners: Vec<Coords>) {
        corners
            .iter()
            .tuple_windows()
            .for_each(|(a, b)| self.add_rock_line_segment(a, b));
    }

    pub fn add_rock_line_segment(&mut self, start: &Coords, end: &Coords) {
        for point in start.points_between_inclusive(end) {
            self.grid.add(Vector::from(point), Rock, false)
        }
    }

    pub fn count_resting_sand(&self) -> usize {
        self.grid.count_visited()
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.grid.fmt(f)
    }
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rock => write!(f, "#"),
            Sand => write!(f, "o"),
            Air => write!(f, "."),
            Source => write!(f, "+"),
        }
    }
}
