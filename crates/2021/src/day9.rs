use std::{collections::BTreeSet, str::FromStr};

use eyre::Result;
use tracing::debug;

use crate::input;

pub fn solve() -> Result<(u64, u64)> {
    let input = input(9);
    let mut heightmap = HeightMap::from_str(&input)?;
    let lowest = heightmap.lowest_points();

    Ok((part1(&lowest), part2(&heightmap)))
}

fn part1(lowest_points: &[u8]) -> u64 {
    lowest_points.iter().map(|lowest| *lowest as u64 + 1).sum()
}

fn part2(heightmap: &HeightMap) -> u64 {
    let mut basins = heightmap.basins();
    basins.sort_by_key(|b| b.size);
    basins.reverse();
    basins.iter().take(3).map(|b| b.size as u64).product()
}

struct Basin {
    size: usize,
}

struct HeightMap {
    map: Vec<Vec<u8>>,
    lowest_points: Vec<(usize, usize)>,
}

impl HeightMap {
    fn basins(&self) -> Vec<Basin> {
        if self.lowest_points.is_empty() {
            panic!("Call lowest points before");
        }
        let mut basins = Vec::with_capacity(self.lowest_points.len());
        for starting_point in &self.lowest_points {
            let mut points_in_basin = BTreeSet::new();
            let mut edge = BTreeSet::from_iter([*starting_point]);
            loop {
                debug!("Trying from start {:?} edge {:?}", starting_point, edge);
                let old_edge = edge;
                let mut new_edge = BTreeSet::new();
                for (x, y) in old_edge {
                    for (x1, y1) in self.neighbor_coords(x, y) {
                        if self.at(x1, y1) == Some(9) {
                            continue;
                        }
                        if points_in_basin.insert((x1, y1)) {
                            new_edge.insert((x1, y1));
                        }
                    }
                }
                if new_edge.is_empty() {
                    break;
                } else {
                    edge = new_edge;
                }
            }
            debug!(
                "Found basin from start {:?}: {:?}",
                starting_point, points_in_basin
            );
            basins.push(Basin {
                size: points_in_basin.len(),
            })
        }
        basins
    }
    fn lowest_points(&mut self) -> Vec<u8> {
        let mut lowest = Vec::new();
        for x in 0..self.height() {
            for y in 0..self.width() {
                let point = self.at(x, y).unwrap();
                let neighbors = self.neighbors(x, y);
                if neighbors.iter().all(|neighbor| point < *neighbor) {
                    // eprintln!("Lowest at ({},{}) ({} < {:?})", x, y, point, neighbors);
                    self.lowest_points.push((x, y));
                    lowest.push(point);
                }
            }
        }
        lowest
    }
    fn height(&self) -> usize {
        self.map.len()
    }
    fn width(&self) -> usize {
        self.map[0].len()
    }
    fn neighbors(&self, x: usize, y: usize) -> Vec<u8> {
        let mut neighbors = Vec::with_capacity(4);
        for (x1, y1) in self.neighbor_coords(x, y) {
            neighbors.extend(self.at(x1, y1))
        }
        neighbors
    }
    fn neighbor_coords(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut coords = Vec::with_capacity(4);
        if x != 0 {
            coords.push((x - 1, y));
        }
        if x != self.height() - 1 {
            coords.push((x + 1, y));
        }
        if y != 0 {
            coords.push((x, y - 1));
        }
        if y != self.width() - 1 {
            coords.push((x, y + 1))
        }

        coords
    }
    fn at(&self, x: usize, y: usize) -> Option<u8> {
        self.map.get(x)?.get(y).copied()
    }
}

impl FromStr for HeightMap {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
            .collect();
        Ok(HeightMap {
            map,
            lowest_points: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9() {
        let (part1, part2) = solve().unwrap();
        assert_eq!(part1, 496);
        assert!(part2 > 30603);
    }

    #[test]
    fn day9_ex() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let mut m = HeightMap::from_str(input).unwrap();
        let part1 = part1(&m.lowest_points());
        assert_eq!(part1, 15);
        assert_eq!(part2(&m), 1134);
    }
}
