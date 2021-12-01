use std::collections::BTreeMap;

use eyre::Result;
use fnv::FnvHashSet;
use tracing::debug;

use crate::input;

pub fn solve() -> Result<(u32, u32)> {
    let input = input(15);
    let grid1 = parse1(&input);
    let grid2 = parse2(&input);

    Ok((grid1.solve(), grid2.solve()))
}

struct Grid {
    inner: Vec<Vec<u8>>,
    start: Coord,
    target: Coord,
}

struct Search {
    visited: FnvHashSet<Coord>,
    queue: Queue,
    target: Coord,
}

#[derive(Default)]
struct Queue(BTreeMap<u32, Vec<(Coord, u32)>>);
impl Queue {
    fn enqueue(&mut self, coord: Coord, cost: u32, weight: u32) {
        self.0.entry(cost + weight).or_default().push((coord, cost));
    }
    fn pop(&mut self) -> Option<(Coord, u32)> {
        let best_cost = *self.0.keys().next()?;
        let best_coords = self.0.get_mut(&best_cost)?;
        let best_coord = best_coords.pop()?;
        if best_coords.is_empty() {
            self.0.remove(&best_cost);
        }
        debug!("Popped {:?} from {:?}", best_coord, self.0);
        Some(best_coord)
    }
}

impl Search {
    fn expand(&mut self, grid: &Grid) -> Option<u32> {
        let (candidate_coord, candidate_cost) = self.queue.pop().unwrap();

        let reached_target = candidate_coord == self.target;
        if reached_target {
            return Some(candidate_cost);
        }
        self.visited.insert(candidate_coord);
        debug!("Exploring {:?} next", candidate_coord);
        for (coord, cost) in grid.neighbors(candidate_coord) {
            if self.visited.contains(&coord) {
                continue;
            }
            if coord == self.target {
                return Some(cost as u32 + candidate_cost);
            }
            self.queue.enqueue(
                coord,
                cost as u32 + candidate_cost,
                manhattan_distance(coord, self.target),
            );
        }
        None
    }
}

fn manhattan_distance(coord: (usize, usize), target: (usize, usize)) -> u32 {
    ((target.0 - coord.0) + (target.1 - coord.1)) as u32
}

impl Grid {
    fn search(&self) -> Search {
        let mut queue = Queue::default();
        for (coord, cost) in self.neighbors(self.start) {
            queue.enqueue(coord, cost as u32, manhattan_distance(coord, self.target));
        }
        Search {
            visited: FnvHashSet::from_iter([(self.start)]),
            queue,
            target: self.target,
        }
    }
    fn solve(&self) -> u32 {
        let mut search = self.search();
        loop {
            if let Some(result) = search.expand(self) {
                return result;
            }
        }
    }
    fn neighbors(&self, coord: Coord) -> Vec<(Coord, u8)> {
        neighbors(coord)
            .into_iter()
            .filter_map(|coord| Some((coord, self.at(coord)?)))
            .collect()
    }
    fn at(&self, (x, y): Coord) -> Option<u8> {
        self.inner.get(x)?.get(y).copied()
    }
}

fn neighbors((x, y): Coord) -> Vec<Coord> {
    [
        (x.saturating_sub(1), y),
        (x + 1, y),
        (x, y.saturating_sub(1)),
        (x, y + 1),
    ]
    .into_iter()
    .filter(|c| *c != (x, y))
    .collect()
}

type Coord = (usize, usize);

fn parse1(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let target = (grid.len() - 1, grid[0].len() - 1);
    Grid {
        inner: grid,
        start: (0, 0),
        target,
    }
}

fn parse2(input: &str) -> Grid {
    let original_grid = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut grid = original_grid.clone();

    fn add_mod(mut x: u8, y: u8) -> u8 {
        for _ in 0..y {
            x += 1;
            if x > 9 {
                x = 1;
            }
        }
        x
    }

    let mut extend_right = |i: u8| {
        for x in 0..grid.len() {
            grid.get_mut(x)
                .unwrap()
                .extend(original_grid.get(x).unwrap().iter().map(|x| add_mod(*x, i)));
        }
    };

    for i in 1..5 {
        extend_right(i);
    }

    let first_line = grid.clone();
    let mut extend_down = |i: u8| {
        grid.extend(
            first_line
                .iter()
                .map(|l| l.iter().map(|x| add_mod(*x, i)).collect()),
        );
    };

    for i in 1..5 {
        extend_down(i);
    }
    let target = (grid.len() - 1, grid[0].len() - 1);
    Grid {
        inner: grid,
        start: (0, 0),
        target,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15() {
        let (part1, part2) = solve().unwrap();
        assert_eq!(part1, 458);
        assert_eq!(part2, 2800);
    }

    #[test]
    fn day15_ex() {
        let example_input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let grid = parse1(example_input);
        assert_eq!(grid.solve(), 40);

        let grid2 = parse2(example_input);
        for line in grid2.inner {
            let s = line.iter().map(|x| (*x + b'0') as char).collect::<String>();
            println!("{}", s);
        }
    }
}
