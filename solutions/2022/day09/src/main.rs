use std::{
    fmt,
    str::FromStr,
    sync::{mpsc, Arc},
};

use shared::{
    read_input, receive_answers, run_part_threaded,
    types_2d::{Direction, InfGrid, Vector},
    ValueError,
};

struct Move {
    direction: Direction,
    steps: usize,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            match self.direction {
                Direction::Up => "U",
                Direction::Down => "D",
                Direction::Left => "L",
                Direction::Right => "R",
            },
            self.steps,
        )
    }
}

impl FromStr for Move {
    type Err = ValueError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let direction = match parts.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(ValueError(s.to_string())),
        };

        Ok(Move {
            direction,
            steps: parts
                .next()
                .unwrap()
                .parse()
                .map_err(|_| ValueError(s.to_string()))?,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Piece(u16);

const HEAD_: Piece = Piece(0b1000000000);
const ONE__: Piece = Piece(0b0100000000);
const TWO__: Piece = Piece(0b0010000000);
const THREE: Piece = Piece(0b0001000000);
const FOUR_: Piece = Piece(0b0000100000);
const FIVE_: Piece = Piece(0b0000010000);
const SIX__: Piece = Piece(0b0000001000);
const SEVEN: Piece = Piece(0b0000000100);
const EIGHT: Piece = Piece(0b0000000010);
const TAIL_: Piece = Piece(0b0000000001);
const ALL__: Piece = Piece(0b1111111111);

const PIECES: [Piece; 10] = [
    HEAD_, ONE__, TWO__, THREE, FOUR_, FIVE_, SIX__, SEVEN, EIGHT, TAIL_,
];

impl Piece {
    fn contains(&self, other: Piece) -> bool {
        self.0 & other.0 > 0
    }

    fn take(&self, other: Piece) -> Piece {
        Piece(!(other.0) & self.0)
    }

    fn add(&self, other: Piece) -> Piece {
        Piece(self.0 | other.0)
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                0b1000000000 => "H",
                0b0100000000 => "1",
                0b0010000000 => "2",
                0b0001000000 => "3",
                0b0000100000 => "4",
                0b0000010000 => "5",
                0b0000001000 => "6",
                0b0000000100 => "7",
                0b0000000010 => "8",
                0b0000000001 => "T",
                // multiple pieces
                _ => "@",
            }
        )
    }
}

fn update_piece(
    grid: &mut InfGrid<Piece>,
    piece: Piece,
    follower: Vector,
    leader: Vector,
    update: bool,
) -> Vector {
    let delta = leader - follower;
    let Vector { x: dx, y: dy } = delta;
    let follow_move = match (dx, dy) {
        // positive dx
        (dx, dy) if dx > 1 => match dy {
            0 => Vector::new(1, 0),
            1 | 2 => Vector::new(1, 1),
            -1 | -2 => Vector::new(1, -1),
            _ => panic!("follower fell behind\nGrid:\n{grid:?}"),
        },
        // negative dx
        (dx, dy) if dx < -1 => match dy {
            0 => Vector::new(-1, 0),
            1 | 2 => Vector::new(-1, 1),
            -1 | -2 => Vector::new(-1, -1),
            _ => panic!("follower fell behind\nGrid:\n{grid:?}"),
        },

        // positive dy
        (dx, dy) if dy > 1 => match dx {
            0 => Vector::new(0, 1),
            1 | 2 => Vector::new(1, 1),
            -1 | -2 => Vector::new(-1, 1),
            _ => panic!("follower fell behind\nGrid:\n{grid:?}"),
        },
        // negative dy
        (dx, dy) if dy < -1 => match dx {
            0 => Vector::new(0, -1),
            1 | 2 => Vector::new(1, -1),
            -1 | -2 => Vector::new(-1, -1),
            _ => panic!("follower fell behind\nGrid:\n{grid:?}"),
        },
        _ => return follower,
    };
    let new_position = follower + follow_move;
    move_piece(grid, piece, follower, new_position, update);
    new_position
}

fn move_piece(grid: &mut InfGrid<Piece>, piece: Piece, from: Vector, to: Vector, visit: bool) {
    // take from
    match grid.get_mut(from) {
        Some(cell) => match cell.value {
            Some(p) => {
                if p.contains(piece) {
                    match p.take(piece) {
                        Piece(0) => cell.value = None,
                        Piece(x) => cell.value = Some(Piece(x)),
                    }
                } else {
                    // if p bit isn't set then panic:
                    panic!("Tried to move a {piece:?} from coords {from:?} but it wasn't there!\nGrid:\n{grid:?}");
                }
            }
            None => panic!("Tried to move a {piece:?} from coords {from:?} but there was nothing there!\nGrid:\n{grid:?}"),
        },
        None => {
            panic!("Tried to move a {piece:?} from coords {from:?} but it was not there!\nGrid:\n{grid:?}");
        }
    };

    // add to
    match grid.get_mut(to) {
        Some(cell) => match cell.value {
            Some(p) if p.contains(piece) => panic!("Tried to move a {piece:?} to coords {from:?} but there it was already there!\nGrid:\n{grid:?}"),
            Some(p) => {
                cell.value = Some(p.add(piece));
                cell.visited = cell.visited || visit;
            }
            None => {
                cell.value = Some(piece);
                cell.visited = cell.visited || visit;
            }
        },
        None => {
            grid.add(to, piece, visit);
        }
    };
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input(2022, 9);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str) -> impl Iterator<Item = Move> + '_ {
    input.lines().map(|line| line.parse().expect("valid input"))
}

fn part1(input: &str) -> usize {
    let mut grid: InfGrid<Piece> = InfGrid::new();

    let mut head = Vector::zero();
    let mut tail = Vector::zero();

    grid.add(head, ALL__, true);

    let moves = parse_input(input);
    for Move { direction, steps } in moves {
        let delta = Vector::from(direction);
        for _ in 0..steps {
            let to = head + delta;
            move_piece(&mut grid, HEAD_, head, to, false);
            head = to;
            tail = update_piece(&mut grid, TAIL_, tail, head, true);
        }
    }
    println!("{grid:?}");

    grid.visited().count()
}

fn part2(input: &str) -> usize {
    let mut grid: InfGrid<Piece> = InfGrid::new();

    let mut positions = [Vector::zero(); 10];

    grid.add(positions[0], ALL__, true);

    let moves = parse_input(input);
    for Move { direction, steps } in moves {
        let delta = Vector::from(direction);
        for _ in 0..steps {
            // move head
            let to = positions[0] + delta;
            move_piece(&mut grid, HEAD_, positions[0], to, false);
            positions[0] = to;

            // update the middle pieces
            for i in 1..9 {
                positions[i] =
                    update_piece(&mut grid, PIECES[i], positions[i], positions[i - 1], false);
            }

            // update the tail, visiting as it goes
            positions[9] = update_piece(&mut grid, TAIL_, positions[9], positions[8], true);
        }
    }
    println!("{grid:?}");

    grid.visited().count()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const LARGER_INPUT: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1);
        assert_eq!(part2(LARGER_INPUT), 36);
    }
}
