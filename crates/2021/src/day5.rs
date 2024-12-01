use std::{collections::BTreeMap, fmt::Debug, str::FromStr};

use eyre::{bail, eyre, Result};
use tracing::debug;

use crate::input;

pub fn solve() -> Result<(usize, usize)> {
    let input = input(5);
    let lines = parse(&input)?;

    let part1 = count_line_intersections(&no_diagonal(&lines));
    let part2 = count_line_intersections(&lines);

    Ok((part1, part2))
}

fn no_diagonal(lines: &[Line]) -> Vec<Line> {
    lines.iter().filter(|l| !l.is_diagonal()).copied().collect()
}

fn count_line_intersections(lines: &[Line]) -> usize {
    let mut points = BTreeMap::new();

    debug!("Lines: {:?}", lines);

    for line in lines {
        for coord in line.points() {
            *points.entry(coord).or_insert(0) += 1;
        }
    }

    points.values().filter(|x| **x >= 2).count()
}

fn parse(input: &str) -> Result<Vec<Line>> {
    let mut result = Vec::new();

    for line in input.lines() {
        let (from, to) = line
            .split_once(" -> ")
            .ok_or_else(|| eyre!("Invalid line {:?}", line))?;
        let from = from.parse()?;
        let to = to.parse()?;
        result.push(Line { from, to });
    }

    Ok(result)
}

#[derive(Debug, Clone, Copy)]
struct Line {
    from: Coord,
    to: Coord,
}

impl Line {
    fn points(&self) -> Vec<Coord> {
        if self.from.x == self.to.x {
            self.from
                .y_range(&self.to)
                .into_iter()
                .map(|y| Coord { x: self.from.x, y })
                .collect()
        } else if self.from.y == self.to.y {
            self.from
                .x_range(&self.to)
                .into_iter()
                .map(|x| Coord { x, y: self.from.y })
                .collect()
        } else {
            assert!(abs_diff(self.from.x, self.to.x) == abs_diff(self.from.y, self.to.y));
            self.from
                .x_range(&self.to)
                .into_iter()
                .zip(self.from.y_range(&self.to))
                .map(|(x, y)| Coord { x, y })
                .collect()
        }
    }
    fn is_diagonal(&self) -> bool {
        self.from.x != self.to.x && self.from.y != self.to.y
    }
}

fn abs_diff(x: u32, y: u32) -> u32 {
    if x > y {
        x - y
    } else {
        y - x
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn x_range(&self, other: &Self) -> Vec<u32> {
        if self.x > other.x {
            (other.x..=self.x).rev().collect()
        } else {
            (self.x..=other.x).collect()
        }
    }
    fn y_range(&self, other: &Self) -> Vec<u32> {
        if self.y > other.y {
            (other.y..=self.y).rev().collect()
        } else {
            (self.y..=other.y).collect()
        }
    }
}

impl FromStr for Coord {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s
            .split(',')
            .map(|x| x.parse::<u32>().map_err(Into::into))
            .collect::<Result<Vec<_>>>()?;

        match *parsed.as_slice() {
            [x, y] => Ok(Coord { x, y }),
            _ => bail!("Invalid Coord {:?}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5() {
        let result = solve().unwrap();

        assert_eq!(result, (8060, 21577));
    }

    #[test]
    fn day5_ex() {
        let _ = tracing_subscriber::fmt::try_init();

        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let lines = parse(input).unwrap();
        let pt1 = count_line_intersections(&no_diagonal(&lines));
        assert_eq!(pt1, 5);
    }

    #[test]
    fn line_fns() {
        let lines = parse("9,4 -> 3,4\n3,4 -> 1,4").unwrap();
        assert_eq!(
            lines[0].points(),
            vec![
                Coord { x: 9, y: 4 },
                Coord { x: 8, y: 4 },
                Coord { x: 7, y: 4 },
                Coord { x: 6, y: 4 },
                Coord { x: 5, y: 4 },
                Coord { x: 4, y: 4 },
                Coord { x: 3, y: 4 },
            ]
        )
    }
}
