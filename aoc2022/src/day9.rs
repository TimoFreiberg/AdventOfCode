use std::{collections::HashSet, ops::AddAssign, str::FromStr};

use tracing::instrument;

use crate::input;

pub fn solve() -> (usize, usize) {
    let input = input(9);
    (part1(&input), part2(&input))
}

fn part1(input: &str) -> usize {
    let moves = parse(input);
    let mut state = State::new(2);
    for m in moves {
        state.apply(m);
    }
    state.visited.len()
}

fn part2(input: &str) -> usize {
    let moves = parse(input);
    let mut state = State::new(10);
    for m in moves {
        state.apply(m);
    }
    state.visited.len()
}

fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(Move::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
}
impl AddAssign<Dir> for Coord {
    fn add_assign(&mut self, dir: Dir) {
        match dir {
            Dir::L => self.x -= 1,
            Dir::R => self.x += 1,
            Dir::U => self.y -= 1,
            Dir::D => self.y += 1,
        }
    }
}
impl Coord {
    #[instrument(ret)]
    fn follow(self, target: Coord) -> Coord {
        use Dir::*;
        let mut new = self;

        let x_dir = match new.x - target.x {
            left if left < -1 => Some(R),
            right if right > 1 => Some(L),
            _ => None,
        };
        if let Some(x_dir) = x_dir {
            new += x_dir;
            match new.y - target.y {
                up if up < 0 => new += D,
                down if down > 0 => new += U,
                _ => {}
            }
        }

        let y_dir = match new.y - target.y {
            up if up < -1 => Some(D),
            down if down > 1 => Some(U),
            _ => None,
        };
        if let Some(y_dir) = y_dir {
            new += y_dir;
            match new.x - target.x {
                left if left < 0 => new += R,
                right if right > 0 => new += L,
                _ => {}
            }
        }

        new
    }
}

struct State {
    rope: Rope,
    visited: HashSet<Coord>,
}

struct Rope {
    positions: Vec<Coord>,
}
impl Rope {
    fn tail(&self) -> Coord {
        *self.positions.last().unwrap()
    }
    fn apply(&mut self, dir: Dir) {
        self.positions[0] += dir;
        for ix in 0..(self.positions.len() - 1) {
            self.positions[ix + 1] = self.positions[ix + 1].follow(self.positions[ix]);
        }
    }
}

impl State {
    fn new(rope_len: usize) -> Self {
        let rope = Rope {
            positions: vec![Coord { x: 0, y: 0 }; rope_len],
        };
        let mut visited = HashSet::new();
        visited.insert(rope.tail());
        Self { rope, visited }
    }
    fn apply(&mut self, m: Move) {
        for _ in 0..m.amount {
            self.rope.apply(m.dir);
            self.visited.insert(self.rope.tail());
        }
    }
}

#[derive(Debug, Clone)]
struct Move {
    dir: Dir,
    amount: usize,
}
impl FromStr for Move {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_once(' ').unwrap();
        Ok(Move {
            dir: dir.parse().unwrap(),
            amount: amount.parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    L,
    R,
    U,
    D,
}

impl FromStr for Dir {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Dir::*;
        Ok(match s {
            "L" => L,
            "R" => R,
            "U" => U,
            "D" => D,
            _ => panic!("Invalid Dir {s:?}"),
        })
    }
}

#[test]
fn example() {
    crate::init();
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
    assert_eq!(part1(input), 13);
}

#[test]
fn follow_diagonally() {
    crate::init();
    let from = Coord { x: 0, y: 0 };
    assert_eq!(from.follow(Coord { x: 2, y: 1 }), Coord { x: 1, y: 1 });
}

#[test]
fn day9() {
    assert_eq!(solve(), (6256, 2665))
}
