use std::{collections::BTreeSet, iter};

use eyre::Result;
use tracing::debug;

use crate::input;

pub fn solve() -> Result<(usize, String)> {
    let input = input(13);
    let (dots, instructions) = parse(&input)?;

    Ok((
        run(dots.clone(), &instructions[..1]).len(),
        dbg_dots(&run(dots, &instructions)),
    ))
}

fn run(mut dots: BTreeSet<Coord>, instructions: &[FoldInstruction]) -> BTreeSet<Coord> {
    debug!("Initial dots:\n{}", dbg_dots(&dots));
    for (direction, pos) in instructions {
        debug!(
            "Folding {} at {}",
            match &direction {
                Direction::Up => "up",
                Direction::Left => "left",
            },
            pos,
        );
        dots = dots
            .into_iter()
            .map(|(x, y)| match direction {
                Direction::Up => (x, fold(y, *pos)),
                Direction::Left => (fold(x, *pos), y),
            })
            .collect();
        debug!("New dots:\n{}", dbg_dots(&dots));
    }
    debug!("Final dots:\n{}", dbg_dots(&dots));
    dots
}

fn dbg_dots(dots: &BTreeSet<Coord>) -> String {
    let xmax = *dots.iter().map(|(x, _)| x).max().unwrap();
    let ymax = *dots.iter().map(|(_, y)| y).max().unwrap();
    let line = iter::repeat('.').take(xmax + 1).collect::<Vec<_>>();
    let mut lines = iter::repeat(line).take(ymax + 1).collect::<Vec<_>>();
    for (x, y) in dots {
        lines[*y][*x] = '#';
    }
    lines
        .into_iter()
        .map(|l| String::from_iter(l) + "\n")
        .collect()
}

fn fold(coord: usize, fold_at: usize) -> usize {
    if coord < fold_at {
        coord
    } else {
        fold_at.checked_sub(coord - fold_at).unwrap()
    }
}

type Coord = (usize, usize);

type FoldInstruction = (Direction, usize);

fn parse(input: &str) -> Result<(BTreeSet<Coord>, Vec<FoldInstruction>)> {
    let (coords, instructions) = input.split_once("\n\n").unwrap();

    let coords = coords
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let instructions = instructions
        .lines()
        .map(|l| {
            let last = l.split_whitespace().last().unwrap();
            let (dir, pos) = last.split_once('=').unwrap();
            (
                match dir {
                    "y" => Direction::Up,
                    "x" => Direction::Left,
                    _ => panic!("Invalid dir {}", dir),
                },
                pos.parse().unwrap(),
            )
        })
        .collect();

    Ok((coords, instructions))
}

enum Direction {
    Up,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13() {
        let (part1, part2) = solve().unwrap();

        assert_eq!(part1, 818);
        assert_eq!(
            part2,
            "\
#....###...##..###..###..####..##..###.
#....#..#.#..#.#..#.#..#.#....#..#.#..#
#....#..#.#....#..#.#..#.###..#....###.
#....###..#.##.###..###..#....#....#..#
#....#.#..#..#.#....#.#..#....#..#.#..#
####.#..#..###.#....#..#.####..##..###.
"
        );
    }

    #[test]
    fn day13_ex() {
        let (dots, instructions) = parse(
            "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5",
        )
        .unwrap();
        let part1 = run(dots, &instructions[..1]).len();

        assert_eq!(part1, 17);
    }
}
