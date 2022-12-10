use std::{
    fmt,
    str::FromStr,
    sync::{mpsc, Arc},
};

use shared::{
    receive_answers, run_part_threaded,
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
enum Piece {
    Head,
    Tail,
    Both,
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Piece::*;
        write!(
            f,
            "{}",
            match self {
                Head => "H",
                Tail => "T",
                Both => "B",
            },
        )
    }
}

fn update_tail(grid: &mut InfGrid<Piece>, tail: Vector, head: Vector) -> Vector {
    let delta = head - tail;
    let Vector { x: dx, y: dy } = delta;
    let tail_move = match (dx, dy) {
        // positive dx
        (dx, dy) if dx > 1 => match dy {
            0 => Vector::new(1, 0),
            1 => Vector::new(1, 1),
            -1 => Vector::new(1, -1),
            _ => panic!("tail fell behind"),
        },
        // negative dx
        (dx, dy) if dx < -1 => match dy {
            0 => Vector::new(-1, 0),
            1 => Vector::new(-1, 1),
            -1 => Vector::new(-1, -1),
            _ => panic!("tail fell behind"),
        },

        // positive dy
        (dx, dy) if dy > 1 => match dx {
            0 => Vector::new(0, 1),
            1 => Vector::new(1, 1),
            -1 => Vector::new(-1, 1),
            _ => panic!("tail fell behind"),
        },
        // negative dy
        (dx, dy) if dy < -1 => match dx {
            0 => Vector::new(0, -1),
            1 => Vector::new(1, -1),
            -1 => Vector::new(-1, -1),
            _ => panic!("tail fell behind"),
        },
        _ => return tail,
    };
    let new_tail = tail + tail_move;
    move_piece(grid, Piece::Tail, tail, tail + tail_move, true);
    new_tail
}

fn move_piece(grid: &mut InfGrid<Piece>, piece: Piece, from: Vector, to: Vector, visit: bool) {
    use Piece::*;
    // take from
    match grid.get_mut(from) {
        Some(cell) => match cell.value {
            Some(p) if p == piece => {
                cell.value = None;
            }
            Some(Both) => {
                cell.value = Some(match piece {
                    Head => Tail,
                    Tail => Head,
                    Both => unreachable!("why are you taking both?"),
                });
            }
            _ => {
                panic!("Tried to move a {piece:?} from coords {from:?} but there was only a Head there!\nGrid:\n{grid:?}");
            }
        },
        None => {
            panic!("Tried to move a {piece:?} from coords {from:?} but it was not there!\nGrid:\n{grid:?}");
        }
    };

    // add to
    match grid.get_mut(to) {
        Some(cell) => match cell.value {
            Some(p) if p == piece || p == Both => panic!("Tried to move a {piece:?} to coords {from:?} but there was already a Head there!\nGrid:\n{grid:?}"),
            Some(_) => {
                cell.value = Some(Both);
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
    let input = std::fs::read_to_string("./input/2022/day9.txt").expect("failed to read input");
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str) -> impl Iterator<Item = Move> + '_ {
    input.lines().map(|line| line.parse().expect("valid input"))
}

fn part1(input: &str) -> usize {
    use Piece::*;
    let mut grid: InfGrid<Piece> = InfGrid::new();

    let mut head = Vector::zero();
    let mut tail = Vector::zero();

    grid.add(head, Both, true);

    let moves = parse_input(input);
    for Move { direction, steps } in moves {
        let delta = Vector::from(direction);
        for _ in 0..steps {
            let to = head + delta;
            move_piece(&mut grid, Head, head, to, false);
            head = to;
            tail = update_tail(&mut grid, tail, head);
        }
    }
    println!("{grid:?}");

    grid.visited().count()
}

fn part2(_input: &str) -> usize {
    todo!()
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 8);
    }
}
