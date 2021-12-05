use std::{collections::BTreeMap, fmt::Debug, str::FromStr};

use eyre::{bail, eyre, Result};
use tracing::debug;

use crate::input;

pub fn solve() -> Result<(usize, ())> {
    let input = input(5);
    let lines = parse(&input)?;

    let part1 = count_line_intersections(only_diagonal(&lines));

    Ok((part1, ()))
}

fn only_diagonal(lines: &[Line]) -> Vec<&Line> {
    lines.iter().filter(|l| !l.is_diagonal()).collect()
}

fn count_line_intersections(lines: Vec<&Line>) -> usize {
    let mut points = BTreeMap::new();

    debug!("Lines: {:?}", lines);

    for line in lines {
        for coord in line.points() {
            *points
                .entry(coord.x)
                .or_insert_with(BTreeMap::new)
                .entry(coord.y)
                .or_insert(0) += 1;
        }
    }

    points
        .values()
        .flat_map(|row| row.values().filter(|x| **x >= 2))
        .count()

    // while let Some(line) = lines.pop() {
    //     for coord in line.points() {
    //         if already_found.contains(&coord) {
    //             continue;
    //         }
    //         let intersecting = lines
    //             .iter()
    //             .filter(|l| l.points().contains(&coord))
    //             .collect::<Vec<_>>();

    //         if !intersecting.is_empty() {
    //             debug!(
    //                 "Intersection at {:?} - with {:?} over {:?}",
    //                 coord, line, intersecting
    //             );
    //             dangerous_points_count += 1;
    //             already_found.insert(coord);
    //         }
    //     }
    // }

    // dangerous_points_count
}

// fn highest(lines: &[Line]) -> (u32, u32) {
//     let mut max_x = 0;
//     let mut max_y = 0;
//     for line in lines {
//         if line.from.x > max_x {
//             max_x = line.from.x;
//         }
//         if line.to.x > max_x {
//             max_x = line.to.x;
//         }
//         if line.from.y > max_y {
//             max_y = line.from.y;
//         }
//         if line.to.y > max_y {
//             max_y = line.to.y;
//         }
//     }
//     (max_x, max_y)
// }

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

#[derive(Debug)]
struct Line {
    from: Coord,
    to: Coord,
}

impl Line {
    fn points(&self) -> Vec<Coord> {
        if self.from.x == self.to.x {
            let y0 = self.from.y.min(self.to.y);
            let y1 = self.from.y.max(self.to.y);
            (y0..=y1).map(|y| Coord { x: self.from.x, y }).collect()
        } else if self.from.y == self.to.y {
            let x0 = self.from.x.min(self.to.x);
            let x1 = self.from.x.max(self.to.x);
            (x0..=x1).map(|x| Coord { x, y: self.from.y }).collect()
        } else {
            assert!(self.from.x - self.to.x == self.from.y - self.to.y);
            vec![]
        }
    }
    fn is_diagonal(&self) -> bool {
        self.from.x != self.to.x && self.from.y != self.to.y
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: u32,
    y: u32,
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

        assert_eq!(result.0, 8060);
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
        let pt1 = count_line_intersections(only_diagonal(&lines));
        assert_eq!(pt1, 5);
    }

    #[test]
    fn line_fns() {
        let lines = parse("9,4 -> 3,4\n3,4 -> 1,4").unwrap();
        assert_eq!(
            lines[0].points(),
            vec![
                Coord { x: 3, y: 4 },
                Coord { x: 4, y: 4 },
                Coord { x: 5, y: 4 },
                Coord { x: 6, y: 4 },
                Coord { x: 7, y: 4 },
                Coord { x: 8, y: 4 },
                Coord { x: 9, y: 4 },
            ]
        )
    }
}
