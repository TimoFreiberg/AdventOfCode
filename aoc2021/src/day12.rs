use std::collections::{BTreeMap, BTreeSet};

use eyre::{eyre, Result};
use tracing::debug;

use crate::input;

pub fn solve() -> Result<(usize, usize)> {
    let input = input(12);

    let map = parse(&input)?;

    Ok((count_paths(&map, false), count_paths(&map, true)))
}

fn count_paths(map: &CaveMap, allow_one_doubledip: bool) -> usize {
    let start = Path::new();
    let mut paths = BTreeSet::from_iter([start]);
    let mut paths_to_end = BTreeSet::new();
    loop {
        debug!("\nPaths: {:#?}", &paths);
        let new_paths = paths
            .into_iter()
            .flat_map(|path| path.next(map, allow_one_doubledip));

        let (ended, running): (BTreeSet<_>, _) = new_paths.partition(|p| p.current_cave == "end");
        paths_to_end.extend(ended);
        if running.is_empty() {
            break;
        } else {
            paths = running;
        }
    }
    debug!("End: {:#?}", paths_to_end);
    paths_to_end.len()
}

struct CaveMap {
    connections: BTreeMap<String, BTreeSet<String>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Path {
    visited_caves: Vec<String>,
    current_cave: String,
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

impl Path {
    fn new() -> Self {
        Self {
            visited_caves: Default::default(),
            current_cave: "start".to_string(),
            visited_small_cave_twice: false,
        }
    }
    fn next(&self, map: &CaveMap, allow_one_doubledip: bool) -> BTreeSet<Path> {
        if let Some(available_paths) = map.connections.get(&self.current_cave) {
            let result = available_paths
                .iter()
                .filter(|&cave| {
                    if cave == "start" {
                        return false;
                    }
                    if is_big(cave) {
                        return true;
                    }
                    if !self.visited_caves.contains(cave) {
                        return true;
                    }
                    if allow_one_doubledip {
                        !self.visited_small_cave_twice
                    } else {
                        false
                    }
                })
                .map(|cave| self.visit(cave.clone()))
                .collect();
            debug!("From {:#?}: {:#?}", self, result);
            result
        } else {
            debug!("No paths available from {}", self.current_cave);
            BTreeSet::new()
        }
    }
    fn visit(&self, cave: String) -> Self {
        let mut visited_caves = self.visited_caves.clone();
        visited_caves.push(self.current_cave.clone());
        let visited_small_cave_twice = if !is_big(&cave) && self.visited_caves.contains(&cave) {
            assert!(
                !self.visited_small_cave_twice,
                "Visiting small cave too often: {:#?}, into {}",
                self, cave
            );
            true
        } else {
            self.visited_small_cave_twice
        };
        Self {
            visited_caves,
            current_cave: cave,
            visited_small_cave_twice,
        }
    }
}

fn parse(input: &str) -> Result<CaveMap> {
    let mut map = BTreeMap::new();

    let mut entry = |k: &str, v: &str| {
        map.entry(k.to_owned())
            .or_insert_with(BTreeSet::new)
            .insert(v.to_owned());
    };

    for line in input.lines() {
        let (from, to) = line
            .split_once('-')
            .ok_or_else(|| eyre!("Invalid line {:?}", line))?;
        entry(from, to);
        entry(to, from);
    }

    Ok(CaveMap { connections: map })
}

fn is_big(cave: &str) -> bool {
    cave.chars().all(|c| c.is_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_ex() {
        let map = parse(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        )
        .unwrap();

        assert_eq!(solve(&map), 10);
    }

    #[test]
    fn day12() {
        let (part1, part2) = solve().unwrap();

        assert_eq!(part1, 4186);
    }
}
