use std::collections::{BTreeMap, BTreeSet};

use eyre::{eyre, Result};

use crate::input;

pub fn solve() -> Result<(usize, usize)> {
    let input = input(12);

    let map = parse(&input)?;

    Ok((
        map.paths(Default::default(), START, Some(START)),
        map.paths(Default::default(), START, None),
    ))
}

impl CaveMap {
    fn paths(&self, mut path: Vec<i32>, current_cave: i32, visited_twice: Option<i32>) -> usize {
        if current_cave == END {
            return 1;
        }
        path.push(current_cave);
        let mut result = 0;
        for next in self.connections[&current_cave]
            .iter()
            .filter(|c| is_big(**c) || !path.contains(c))
        {
            result += self.paths(path.clone(), *next, visited_twice);
        }
        if visited_twice.is_none() {
            for next in self.connections[&current_cave]
                .iter()
                .filter(|c| !is_big(**c) && path.contains(c) && **c != START)
            {
                result += self.paths(path.clone(), *next, Some(*next));
            }
        }
        result
    }
}

struct CaveMap {
    connections: BTreeMap<i32, BTreeSet<i32>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Path {
    visited_caves: Vec<i32>,
    current_cave: i32,
    visited_small_cave_twice: bool,
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cave in &self.visited_caves {
            write!(f, "{},", cave)?;
        }
        write!(f, "{}", self.current_cave)
    }
}

const START: i32 = 0;
const END: i32 = 1;

fn parse<'a>(input: &'a str) -> Result<CaveMap> {
    let mut map = BTreeMap::new();

    let mut entry = |k: i32, v: i32| {
        map.entry(k.to_owned())
            .or_insert_with(BTreeSet::new)
            .insert(v.to_owned());
    };
    let mut interning = BTreeMap::new();
    let mut counter = 2;

    interning.insert("start", START);
    interning.insert("end", END);

    let mut intern = |s: &'a str| match interning.get(s) {
        Some(it) => *it,
        None => {
            let val = if s.chars().all(|c| c.is_uppercase()) {
                -counter
            } else {
                counter
            };
            interning.insert(s, val);
            counter += 1;
            val
        }
    };

    for line in input.lines() {
        let (from, to) = line
            .split_once('-')
            .ok_or_else(|| eyre!("Invalid line {:?}", line))?;
        let from = intern(from);
        let to = intern(to);
        entry(from, to);
        entry(to, from);
    }

    Ok(CaveMap { connections: map })
}

/// Negative cave ids are used for big caves
fn is_big(cave: i32) -> bool {
    cave < 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12() {
        let (part1, part2) = solve().unwrap();

        assert_eq!(part1, 4186);
        assert_eq!(part2, 92111);
    }
}
