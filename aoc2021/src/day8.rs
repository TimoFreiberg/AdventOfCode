use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use eyre::{bail, eyre, Result};
use itertools::Itertools;
use tracing::debug;

use crate::input;

pub fn solve() -> Result<(usize, u32)> {
    let input = input(8);
    let displays = parse(&input)?;

    Ok((part1(&displays), part2(&displays)))
}

fn part1(displays: &[Display]) -> usize {
    displays.iter().map(|d| d.obvious_numbers().len()).sum()
}

fn part2(displays: &[Display]) -> u32 {
    displays
        .iter()
        .map(|display| display.brute_force().unwrap())
        .sum()
}

fn parse(input: &str) -> Result<Vec<Display>> {
    input.lines().map(Display::from_str).collect()
}

#[derive(Debug)]
struct Display {
    signal_patterns: Vec<String>,
    output: Vec<String>,
}

impl Display {
    fn obvious_numbers(&self) -> Vec<u8> {
        self.output
            .iter()
            .filter_map(|s| {
                let matching = length_mapping(s.len());
                if matching.len() == 1 {
                    Some(matching[0])
                } else {
                    None
                }
            })
            .collect()
    }

    fn brute_force(&self) -> Result<u32> {
        let one = self
            .signal_patterns
            .iter()
            .find(|pat| pat.len() == 2)
            .unwrap();
        let seven = self
            .signal_patterns
            .iter()
            .find(|pat| pat.len() == 3)
            .unwrap();
        let mapped_to_a = seven.chars().find(|c| !one.contains(*c)).unwrap();

        let from = Vec::from_iter('a'..='g');
        let mut possible_mapping = Vec::new();
        for from_char in from {
            if from_char == mapped_to_a {
                possible_mapping.push(BTreeSet::from_iter(['a']));
                continue;
            }
            let signals_containing_from = self
                .signal_patterns
                .iter()
                .filter(|s| s.contains(from_char));
            let possible_mappings_for_from = signals_containing_from
                .flat_map(|s| length_mapping(s.len()))
                .map(char_mapping)
                .flat_map(|s| s.chars())
                .collect::<BTreeSet<_>>();
            possible_mapping.push(possible_mappings_for_from);
        }

        for to_permutation in ('a'..='g').permutations(7).filter(|permutation| {
            permutation
                .iter()
                .zip(&possible_mapping)
                .all(|(mapping, possible_mappings)| possible_mappings.contains(mapping))
        }) {
            let from = 'a'..='g';
            let mapping = from.zip(to_permutation).collect();
            if self.valid_mapping(&mapping) {
                let result = self
                    .output
                    .iter()
                    .map(|s| signal_mapping(&apply_mapping(s, &mapping).unwrap()).to_string())
                    .collect::<String>();
                return Ok(result.parse()?);
            }
        }
        bail!("No mapping found :(");
    }

    fn valid_mapping(&self, mapping: &BTreeMap<char, char>) -> bool {
        self.signal_patterns
            .iter()
            .all(|s| match apply_mapping(s, mapping) {
                Some(mapped) => {
                    let result = valid_signal(&sort_string(&mapped));
                    if !result {
                        debug!(
                            "Invalid signal {:?} (mapped from {:?})",
                            sort_string(&mapped),
                            sort_string(s)
                        );
                    }
                    result
                }
                None => {
                    debug!("Incomplete mapping, {:?} couldn't be mapped", s);
                    false
                }
            })
    }
}

fn apply_mapping(s: &str, mapping: &BTreeMap<char, char>) -> Option<String> {
    let mut result = Vec::with_capacity(s.len());
    for c in s.chars() {
        result.push(*mapping.get(&c)?);
    }
    result.sort_unstable();
    Some(String::from_iter(result))
}

fn valid_signal(signal: &str) -> bool {
    matches!(
        signal,
        "abcefg"
            | "cf"
            | "acdeg"
            | "acdfg"
            | "bcdf"
            | "abdfg"
            | "abdefg"
            | "acf"
            | "abcdefg"
            | "abcdfg"
    )
}

fn signal_mapping(signal: &str) -> u8 {
    match signal {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => unreachable!("Illegal signal {:?}", signal),
    }
}

fn char_mapping(digit: u8) -> &'static str {
    match digit {
        0 => "abcefg",
        1 => "cf",
        2 => "acdeg",
        3 => "acdfg",
        4 => "bcdf",
        5 => "abdfg",
        6 => "abdefg",
        7 => "acf",
        8 => "abcdefg",
        9 => "abcdfg",
        _ => unreachable!("Illegal digit {:?}", digit),
    }
}

fn length_mapping(len: usize) -> Vec<u8> {
    vec![match len {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => return vec![2, 3, 5],
        6 => return vec![0, 6, 9],
        7 => 8,
        _ => unreachable!("Invalid length {}", len),
    }]
}

impl FromStr for Display {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signal_patterns, output) = s
            .split_once('|')
            .ok_or_else(|| eyre!("No pipe found in {:?}", s))?;
        let signal_patterns = signal_patterns
            .trim()
            .split_whitespace()
            .map(ToString::to_string)
            .collect();
        let output = output
            .trim()
            .split_whitespace()
            .map(ToString::to_string)
            .collect();
        Ok(Display {
            signal_patterns,
            output,
        })
    }
}

fn sort_string(s: &str) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort_unstable();
    String::from_iter(chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8() {
        let (part1, part2) = solve().unwrap();
        assert_eq!(part1, 288);
        assert_eq!(part2, 940724);
    }

    #[test]
    fn day8_example() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let displays = parse(input).unwrap();
        let display = &displays[0];
        let dfs_result = display.brute_force();

        assert_eq!(dfs_result.unwrap(), 5353);
    }
}
