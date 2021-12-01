use tap::Tap;
use tracing::debug;

use crate::input;

pub fn solve() -> (usize, usize) {
    let input = input(8);
    (part1(&input), part2(&input))
}

type Coord = (usize, usize);

fn part1(input: &str) -> usize {
    let grid = Grid::parse(input);
    let mut visible = 0;
    for tree_coord in grid.all_tree_coords() {
        let tree = grid.tree_at(tree_coord).unwrap();
        debug!(?tree_coord, ?tree, "testing");
        if [
            ("up", up(tree_coord)),
            ("down", down(tree_coord)),
            ("left", left(tree_coord)),
            ("right", right(tree_coord)),
        ]
        .into_iter()
        .any(|(dir, line)| {
            line.take_while(|coord| grid.tree_at(*coord).is_some())
                .all(|coord| grid.tree_at(coord).unwrap() < tree)
                .tap(|visible| {
                    if *visible {
                        debug!(from = ?dir, ?tree_coord, ?tree, "visible");
                    }
                })
        }) {
            visible += 1;
        }
    }
    visible
}

fn part2(input: &str) -> usize {
    let grid = Grid::parse(input);
    grid.all_tree_coords()
        .map(|coord| grid.scenic_score(coord))
        .max()
        .unwrap()
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn parse(input: &str) -> Self {
        Grid(input.lines().map(|l| l.chars().collect()).collect())
    }
    fn all_tree_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.0.len()).flat_map(move |x| (0..self.0[x].len()).map(move |y| (x, y)))
    }
    fn tree_at(&self, coord: Coord) -> Option<char> {
        let (x, y) = coord;
        self.0
            .get(x)
            .and_then(|row: &Vec<char>| row.get(y))
            .copied()
    }
    fn scenic_score(&self, coord: Coord) -> usize {
        [
            ("up", up(coord)),
            ("down", down(coord)),
            ("left", left(coord)),
            ("right", right(coord)),
        ]
        .into_iter()
        .map(|(dir, line)| {
            self.viewing_distance(coord, line)
                .tap(|dist| debug!(?coord, ?dir, ?dist))
        })
        .product::<usize>()
        .tap(|score| debug!(?coord, ?score))
    }

    fn viewing_distance(&self, coord: Coord, line: Box<dyn Iterator<Item = Coord>>) -> usize {
        let tree = self.tree_at(coord).unwrap();
        let mut dist = 0;
        for next_coord in line.take_while(|coord| self.tree_at(*coord).is_some()) {
            dist += 1;
            if self.tree_at(next_coord).unwrap() >= tree {
                break;
            }
        }
        dist
    }
}

fn up((x, y): Coord) -> Box<dyn Iterator<Item = Coord>> {
    Box::new((0..x).rev().map(move |x| (x, y)))
}

fn down((x, y): Coord) -> Box<dyn Iterator<Item = Coord>> {
    Box::new((x + 1..).map(move |x| (x, y)))
}

fn left((x, y): Coord) -> Box<dyn Iterator<Item = Coord>> {
    Box::new((0..y).rev().map(move |y| (x, y)))
}

fn right((x, y): Coord) -> Box<dyn Iterator<Item = Coord>> {
    Box::new((y + 1..).map(move |y| (x, y)))
}

#[test]
fn example() {
    crate::init();
    let input = "\
30373
25512
65332
33549
35390";
    assert_eq!(part1(input), 21);
    assert_eq!(part2(input), 8);
}

#[test]
fn day8() {
    assert_eq!(solve(), (1794, 199272))
}
