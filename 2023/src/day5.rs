use std::{collections::HashMap, str::FromStr};

use eyre::{bail, eyre, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::input;

pub fn solve() -> (u64, u64) {
    let input = input(5);
    let almanac: Almanac = input.parse().unwrap();

    (part1(&almanac), part2(&almanac))
}

fn part1(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .copied()
        .map(|seed| almanac.location(seed))
        .min()
        .unwrap()
}

fn part2(almanac: &Almanac) -> u64 {
    let mut seeds = Vec::new();
    for range in almanac.seeds.chunks_exact(2) {
        let start = range[0];
        let length = range[1];
        seeds.extend(start..(start + length));
    }
    seeds
        .par_iter()
        .copied()
        .map(|seed| almanac.location(seed))
        .min()
        .unwrap()
}

struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, Map>,
}

impl Almanac {
    fn location(&self, seed: u64) -> u64 {
        let mut thing = "seed".to_string();
        let mut num = seed;
        loop {
            if thing == "location" {
                return num;
            }
            (num, thing) = self.lookup(&thing, num);
        }
    }
    fn lookup(&self, thing: &str, num: u64) -> (u64, String) {
        let map = &self.maps.get(thing).expect(&format!("no map for {thing}"));
        let result_num = map.convert(num);
        (result_num, map.destination.clone())
    }
}

impl FromStr for Almanac {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut paragraphs = s.split("\n\n");
        let seeds = paragraphs.next().ok_or(eyre!("no seeds paragraph"))?;
        let seeds = seeds
            .strip_prefix("seeds: ")
            .ok_or(eyre!("no seeds prefix"))?
            .split_whitespace()
            .map(|s| Ok(s.parse()?))
            .collect::<Result<Vec<_>>>()?;

        let mut maps = HashMap::new();
        for paragraph in paragraphs {
            let map: Map = paragraph.parse()?;
            maps.insert(map.source.clone(), map);
        }

        Ok(Almanac { seeds, maps })
    }
}

struct Map {
    source: String,
    destination: String,
    ranges: Vec<MapRange>,
}
impl Map {
    fn convert(&self, num: u64) -> u64 {
        if let Some(range) = self
            .ranges
            .iter()
            .find(|range| (range.source_start..(range.source_start + range.length)).contains(&num))
        {
            let offset = num - range.source_start;
            return range.destination_start + offset;
        }
        num
    }
}

impl FromStr for Map {
    type Err = eyre::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = lines.next().ok_or(eyre!("no first line"))?;
        let mapping_string = header.trim_end_matches(" map:");
        let (source, destination) = mapping_string.split_once("-to-").ok_or(eyre!("no -to-"))?;
        let ranges = lines.map(|line| Ok(line.parse()?)).collect::<Result<_>>()?;
        Ok(Map {
            source: source.to_string(),
            destination: destination.to_string(),
            ranges,
        })
    }
}

struct MapRange {
    source_start: u64,
    destination_start: u64,
    length: u64,
}
impl FromStr for MapRange {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split_whitespace()
            .map(|s| Ok(s.parse()?))
            .collect::<Result<Vec<_>>>()?;
        match nums.as_slice() {
            &[destination_start, source_start, length] => Ok(MapRange {
                source_start,
                destination_start,
                length,
            }),
            _ => {
                bail!("bad range format: {s:?}")
            }
        }
    }
}
