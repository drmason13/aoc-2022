use std::{
    collections::HashMap,
    fmt,
    ops::{Add, Mul, MulAssign, Sub},
};

#[cfg(feature = "itertools")]
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

impl Coords {
    pub fn zero() -> Self {
        Coords { x: 0, y: 0 }
    }
}

#[cfg(feature = "itertools")]
/// iter every Coord from left to right and top to bottom: (0, 0) is first and represents the top left
pub fn iter_coords(dimensions: &Size) -> impl Iterator<Item = Coords> {
    (0..dimensions.width)
        .cartesian_product(0..dimensions.height)
        .map(|(x, y)| Coords { x, y })
}

#[cfg(feature = "itertools")]
/// iter every position Vector from left to right and top to bottom: (0, 0) is first and represents the top left
pub fn iter_positions(top_left: &Vector, bottom_right: &Vector) -> impl Iterator<Item = Vector> {
    (top_left.y..=bottom_right.y)
        .cartesian_product(top_left.x..=bottom_right.x)
        // to order left-right, top bottom this ordering works out
        .map(|(y, x)| Vector {
            x: x as isize,
            y: y as isize,
        })
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

impl Vector {
    pub fn new(x: isize, y: isize) -> Self {
        Vector { x, y }
    }
    pub fn zero() -> Self {
        Vector { x: 0, y: 0 }
    }
}

impl From<Direction> for Vector {
    fn from(direction: Direction) -> Self {
        use Direction::*;
        match direction {
            Up => Vector { x: 0, y: -1 },
            Down => Vector { x: 0, y: 1 },
            Left => Vector { x: -1, y: 0 },
            Right => Vector { x: 1, y: 0 },
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<isize> for Vector {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<isize> for Vector {
    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl From<Coords> for Vector {
    fn from(other: Coords) -> Self {
        Vector {
            x: other.x as isize,
            y: other.y as isize,
        }
    }
}

impl TryFrom<Vector> for Coords {
    type Error = OutOfBounds;

    fn try_from(value: Vector) -> Result<Self, Self::Error> {
        Coords::zero() + value
    }
}

/// the negative coords one or both of x, y are negative
#[derive(Clone, Copy, Debug)]
pub struct OutOfBounds(Vector);

impl Add<Vector> for Coords {
    type Output = Result<Coords, OutOfBounds>;

    fn add(self, rhs: Vector) -> Self::Output {
        let x = self.x as isize + rhs.x as isize;
        let y = self.y as isize + rhs.y as isize;

        if x < 0 || y < 0 {
            Err(OutOfBounds(Vector { x, y }))
        } else {
            Ok(Coords {
                x: x as usize,
                y: y as usize,
            })
        }
    }
}

pub fn modulus(n: isize) -> usize {
    match n {
        n if n < 0 => -n as usize,
        _ => n as usize,
    }
}

impl Sub<Coords> for Coords {
    type Output = Vector;

    fn sub(self, rhs: Coords) -> Self::Output {
        let x = self.x as isize - rhs.x as isize;
        let y = self.y as isize - rhs.y as isize;
        Vector { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    /// create a new size of given width and height
    pub fn new(width: usize, height: usize) -> Self {
        Size { width, height }
    }

    /// create a new zero size
    pub fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }
}

impl From<Size> for Coords {
    fn from(s: Size) -> Self {
        Coords {
            x: s.width,
            y: s.height,
        }
    }
}

impl From<Coords> for Size {
    fn from(c: Coords) -> Self {
        Size {
            width: c.x,
            height: c.y,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GridCell<T> {
    pub value: Option<T>,
    pub visited: bool,
}

impl<T> Default for GridCell<T> {
    fn default() -> Self {
        GridCell {
            value: None,
            visited: false,
        }
    }
}

impl<T> GridCell<T> {
    pub fn new(value: T) -> Self {
        GridCell {
            value: Some(value),
            visited: false,
        }
    }
}

pub struct InfGrid<T> {
    cells: HashMap<Vector, GridCell<T>>,
    top_left: Vector,
    bottom_right: Vector,
}

impl<T: fmt::Debug> fmt::Debug for InfGrid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InfGrid {{ top_left: {:?}, bottom_right: {:?} }}",
            self.top_left, self.bottom_right
        )?;

        writeln!(f)?;
        let max_x = self.bottom_right.x;
        for position in self.positions() {
            write!(
                f,
                "{}",
                self.get(position)
                    .map(|x| match &x.value {
                        Some(x) => format!("{x:?}"),
                        None => String::from(","),
                    })
                    .unwrap_or_else(|| ".".into())
            )?;
            if position.x == max_x {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<T> InfGrid<T> {
    pub fn new() -> Self {
        InfGrid {
            cells: HashMap::new(),
            top_left: Vector { x: 0, y: 0 },
            bottom_right: Vector { x: 0, y: 0 },
        }
    }

    pub fn get(&self, position: Vector) -> Option<&GridCell<T>> {
        self.cells.get(&position)
    }

    pub fn get_mut(&mut self, position: Vector) -> Option<&mut GridCell<T>> {
        self.cells.get_mut(&position)
    }

    pub fn add(&mut self, position: Vector, value: T, visited: bool) {
        self.cells.insert(
            position,
            GridCell {
                value: Some(value),
                visited,
            },
        );
        if position.x < self.top_left.x {
            self.top_left.x = position.x;
        }
        if position.x > self.bottom_right.x {
            self.bottom_right.x = position.x;
        }
        if position.y < self.top_left.y {
            self.top_left.y = position.y;
        }
        if position.y > self.bottom_right.y {
            self.bottom_right.y = position.y;
        }
    }

    #[cfg(feature = "itertools")]
    /// iter every Coord from left to right and top to bottom
    pub fn positions(&self) -> impl Iterator<Item = Vector> {
        iter_positions(&self.top_left, &self.bottom_right)
    }
}

impl<T: Clone> InfGrid<T> {
    pub fn visited(&self) -> impl Iterator<Item = GridCell<T>> + '_ {
        self.cells.values().filter(|cell| cell.visited).cloned()
    }
}

impl<T> Default for InfGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_positions_order() {
        let grid: InfGrid<()> = InfGrid {
            cells: HashMap::new(),
            top_left: Vector { x: -2, y: -1 },
            bottom_right: Vector { x: 0, y: 1 },
        };

        let mut positions = grid.positions();
        assert_eq!(positions.next().unwrap(), grid.top_left);
        assert_eq!(positions.next().unwrap(), Vector { x: -1, y: -1 });
        assert_eq!(positions.next().unwrap(), Vector { x: 0, y: -1 });
        assert_eq!(positions.next().unwrap(), Vector { x: -2, y: 0 });
        assert_eq!(positions.next().unwrap(), Vector { x: -1, y: 0 });
        assert_eq!(positions.next().unwrap(), Vector { x: 0, y: 0 });
        assert_eq!(positions.next().unwrap(), Vector { x: -2, y: 1 });
        assert_eq!(positions.next().unwrap(), Vector { x: -1, y: 1 });
        assert_eq!(positions.next().unwrap(), Vector { x: 0, y: 1 });
    }

    #[test]
    fn test_grid_positions_when_empty() {
        let grid: InfGrid<()> = InfGrid {
            cells: HashMap::new(),
            top_left: Vector::zero(),
            bottom_right: Vector::zero(),
        };

        let total = grid.positions().count();
        assert_eq!(total, 1);
    }

    #[test]
    fn test_grid_positions_when_square() {
        let grid: InfGrid<()> = InfGrid {
            cells: HashMap::new(),
            top_left: Vector { x: -2, y: -1 },
            bottom_right: Vector { x: 0, y: 1 },
        };

        let total = grid.positions().count();
        assert_eq!(total, 9);
    }

    struct Foo;
    impl fmt::Debug for Foo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "+")
        }
    }

    #[test]
    fn test_grid_positions_debug() {
        let mut grid: InfGrid<Foo> = InfGrid::new();

        grid.add(Vector { x: -2, y: -1 }, Foo, false);
        grid.add(Vector { x: -1, y: 0 }, Foo, false);
        grid.add(Vector { x: 0, y: 1 }, Foo, false);

        assert_eq!(
            format!("{grid:?}"),
            String::from(
                "\
InfGrid { top_left: Vector { x: -2, y: -1 }, bottom_right: Vector { x: 0, y: 1 } }
+..
.+.
..+
"
            )
        );
    }
}
