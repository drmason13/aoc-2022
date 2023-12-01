use shared::types_2d::OutOfBounds;
use shared::types_2d::{Coords, Vector};

use crate::map::Map;

#[derive(Clone, Copy)]
pub enum Entity {
    #[allow(unused)]
    Air,
    Rock,
    Sand,
    Source,
}

use Entity::*;

pub struct FallingSand {
    position: Coords,
}

impl FallingSand {
    pub fn new(source: Coords) -> Self {
        FallingSand { position: source }
    }
    pub fn fall(&mut self, map: &Map) -> Result<Coords, OutOfBounds> {
        if let Some(next) = self.test_fall(Vector::new(0, 1), map)? {
            self.position = next;
            self.fall(map)
        } else if let Some(next) = self.test_fall(Vector::new(-1, 1), map)? {
            self.position = next;
            self.fall(map)
        } else if let Some(next) = self.test_fall(Vector::new(1, 1), map)? {
            self.position = next;
            self.fall(map)
        } else {
            // this is the position of the sand at rest
            Ok(self.position)
        }
    }

    pub fn test_fall(&self, to: Vector, map: &Map) -> Result<Option<Coords>, OutOfBounds> {
        let position = (self.position + to)?;
        match map.get(position)? {
            Some(Rock) | Some(Sand) => Ok(None),
            _ => Ok(Some(position)),
        }
    }
}
