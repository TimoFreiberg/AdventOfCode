use fxhash::FxHashSet;
use std::collections::BTreeMap;
use tracing::debug;

use crate::input;

pub fn solve() -> (usize, usize) {
    let input = input(12);
    (part1(&input), 0)
}

fn part1(input: &str) -> usize {
    let mut grid = Search::parse(input);
    grid.find_path()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
impl Coord {
    fn neighbors(&self) -> Vec<Coord> {
        let Coord { x, y } = *self;
        vec![
            Coord { x: x + 1, y },
            Coord {
                x: x.wrapping_sub(1),
                y,
            },
            Coord { x, y: y + 1 },
            Coord {
                x,
                y: y.wrapping_sub(1),
            },
        ]
    }
    fn dist(&self, target: Coord) -> usize {
        fn udist(x: usize, y: usize) -> usize {
            if x > y {
                x - y
            } else {
                y - x
            }
        }
        udist(self.x, target.x) + udist(self.y, target.y)
    }
}

struct Search {
    grid: Vec<Vec<char>>,
    start: Coord,
    target: Coord,
    visited: FxHashSet<Coord>,
    queue: BTreeMap<usize, Vec<Coord>>,
    came_from: BTreeMap<Coord, Coord>,
}

impl Search {
    fn find_path(&mut self) -> usize {
        loop {
            let explored = self.visit_next();
            if explored == self.target {
                return self.path_len();
            }
        }
    }
    fn path_len(&self) -> usize {
        let mut path_len = 0;
        let mut coord = self.target;
        loop {
            coord = *self.came_from.get(&coord).unwrap();
            if coord == self.start {
                return path_len;
            }
            path_len += 1;
        }
    }
    fn reachable_neighbors(&self, coord: Coord) -> Vec<Coord> {
        let current_elevation = self.get(coord).unwrap();
        coord
            .neighbors()
            .into_iter()
            .filter(|neighbor| !self.visited.contains(neighbor))
            .filter_map(move |neighbor| {
                let neighbor_val = self.get(neighbor)?;
                if elevation(neighbor_val) <= next_char(current_elevation) {
                    debug!(
                        neigh = ?neighbor,
                        neigh_ele = ?neighbor_val,
                        from = ?coord,
                        from_ele  = ?current_elevation,
                        nc = ?next_char(current_elevation),
                        "ok"
                    );
                    Some(neighbor)
                } else {
                    None
                }
            })
            .collect()
    }
    fn visit_next(&mut self) -> Coord {
        let next = self.pop();
        debug!(?next, from = ?self.came_from.get(&next));
        self.visited.insert(next);
        let next_neighbors = self.reachable_neighbors(next);
        self.track_came_from(next, &next_neighbors);
        self.enqueue(&next_neighbors);
        next
    }
    fn enqueue(&mut self, coords: &[Coord]) {
        for coord in coords {
            let dist = coord.dist(self.target);
            self.queue.entry(dist).or_default().push(*coord);
        }
    }
    fn pop(&mut self) -> Coord {
        let mut entry = self.queue.first_entry().unwrap();
        let queue = entry.get_mut();
        let next = queue.pop().unwrap();
        if queue.is_empty() {
            entry.remove();
        }
        next
    }
    fn init_queue(&mut self) {
        let reachable_neighbors = self.reachable_neighbors(self.start);
        self.visited.insert(self.start);
        self.enqueue(&reachable_neighbors);
        self.track_came_from(self.start, &reachable_neighbors);
    }
    fn get(&self, coord: Coord) -> Option<char> {
        let row = self.grid.get(coord.x)?;
        row.get(coord.y).copied()
    }
    fn parse(input: &str) -> Self {
        let mut start = None;
        let mut target = None;
        let grid = input
            .lines()
            .enumerate()
            .map(|(x, l)| {
                l.chars()
                    .enumerate()
                    .map(|(y, c)| {
                        match c {
                            'S' => start = Some(Coord { x, y }),
                            'E' => target = Some(Coord { x, y }),
                            _ => {}
                        }
                        c
                    })
                    .collect()
            })
            .collect();
        let mut grid = Self {
            grid,
            start: start.unwrap(),
            target: target.unwrap(),
            visited: Default::default(),
            queue: Default::default(),
            came_from: Default::default(),
        };
        grid.init_queue();
        grid
    }

    fn track_came_from(&mut self, next: Coord, next_neighbors: &[Coord]) {
        for neighbor in next_neighbors {
            self.came_from.insert(*neighbor, next);
        }
    }
}

fn elevation(c: char) -> char {
    match c {
        'S' => 'a',
        'E' => 'z',
        lower if lower.is_lowercase() => lower,
        other => panic!("Only lowercase chars and S/E allowed, got {other:?}"),
    }
}

fn next_char(c: char) -> char {
    char::from_u32((elevation(c) as u32 + 1).min('z' as u32)).unwrap()
}

#[test]
fn example() {
    crate::init();
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(part1(&input), 31);
}
