use std::str::FromStr;

use eyre::{bail, Result};

use crate::input;

pub fn solve() -> Result<(u32, u32)> {
    let input = input(2);
    let directions = input
        .lines()
        .map(Dir::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok((part1(&directions), part2(&directions)))
}

fn part1(directions: &[Dir]) -> u32 {
    let mut pos = 0;
    let mut depth = 0;
    for dir in directions {
        match dir {
            Dir::Forward(x) => pos += x,
            Dir::Down(x) => depth += x,
            Dir::Up(x) => depth -= x,
        }
    }
    pos * depth
}

fn part2(directions: &[Dir]) -> u32 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for dir in directions {
        match dir {
            Dir::Forward(x) => {
                pos += x;
                depth += aim * x;
            }
            Dir::Down(x) => aim += x,
            Dir::Up(x) => aim -= x,
        }
    }
    pos * depth
}

enum Dir {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Dir {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(x) = s.strip_prefix("forward ") {
            Dir::Forward(x.parse()?)
        } else if let Some(x) = s.strip_prefix("down ") {
            Dir::Down(x.parse()?)
        } else if let Some(x) = s.strip_prefix("up ") {
            Dir::Up(x.parse()?)
        } else {
            bail!("Unknown direction {:?}", s)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2() {
        assert_eq!(solve().unwrap(), (1882980, 1971232560))
    }
}
