use std::{collections::BTreeMap, str::FromStr};

use eyre::{eyre, Result};

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
    displays.iter().map(|display| display.part2_puzzle()).sum()
}

fn parse(input: &str) -> Result<Vec<Display>> {
    input.lines().map(Display::from_str).collect()
}

#[derive(Debug)]
struct Display {
    signal_patterns: Vec<Signal>,
    output: Vec<Signal>,
}

impl Display {
    fn obvious_numbers(&self) -> Vec<u8> {
        self.output
            .iter()
            .filter_map(|s| {
                let matching = length_mapping(s.0.len());
                if matching.len() == 1 {
                    Some(matching[0])
                } else {
                    None
                }
            })
            .collect()
    }

    fn part2_puzzle(&self) -> u32 {
        let one = self.signal_patterns_with_length(2)[0];
        let four = self.signal_patterns_with_length(4)[0];
        let seven = self.signal_patterns_with_length(3)[0];
        let eight = self.signal_patterns_with_length(7)[0];

        let a = seven.difference(one);

        let nine = self
            .signal_patterns_with_length(6)
            .into_iter()
            .filter(|s| s.difference(four).difference(&a).0.len() == 1)
            .collect::<Vec<_>>();
        assert_eq!(nine.len(), 1, "Only one 'nine': {:?}", nine);

        let nine = nine[0];

        let two_three_five = self.signal_patterns_with_length(5);

        let two = two_three_five
            .iter()
            .filter(|s| !s.difference(nine).0.is_empty())
            .collect::<Vec<_>>();

        assert_eq!(two.len(), 1, "Two: {:?}", two);

        let two = two[0];

        let three = two_three_five
            .iter()
            .filter(|s| s.difference(two).0.len() == 1)
            .collect::<Vec<_>>();

        assert_eq!(three.len(), 1, "Three: {:?}", three);

        let three = three[0];

        let five = two_three_five
            .iter()
            .filter(|&s| s != two && s != three)
            .collect::<Vec<_>>();

        assert_eq!(five.len(), 1, "Five: {:?}", five);

        let five = five[0];

        let six = self
            .signal_patterns_with_length(6)
            .into_iter()
            .filter(|s| eight.difference(s).intersection(one).0.len() == 1)
            .collect::<Vec<_>>();

        assert_eq!(six.len(), 1, "Six: {:?}", six);

        let six = six[0];

        let zero = self
            .signal_patterns_with_length(6)
            .into_iter()
            .filter(|&s| s != six && s != nine)
            .collect::<Vec<_>>();

        assert_eq!(zero.len(), 1, "Zero: {:?}", zero);

        let zero = zero[0];

        let mapping = BTreeMap::from_iter([
            (zero, '0'),
            (one, '1'),
            (two, '2'),
            (three, '3'),
            (four, '4'),
            (five, '5'),
            (six, '6'),
            (seven, '7'),
            (eight, '8'),
            (nine, '9'),
        ]);

        let result = self
            .output
            .iter()
            .map(|signal| mapping[signal])
            .collect::<String>();

        result.parse().unwrap()
    }

    fn signal_patterns_with_length(&self, len: usize) -> Vec<&Signal> {
        self.signal_patterns_matching(|s| s.0.len() == len)
    }

    fn signal_patterns_matching(&self, pred: impl Fn(&Signal) -> bool) -> Vec<&Signal> {
        self.signal_patterns.iter().filter(|s| pred(s)).collect()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Signal(String);

impl Signal {
    fn difference(&self, other: &Self) -> Self {
        Signal(self.0.chars().filter(|c| !other.0.contains(*c)).collect())
    }
    fn intersection(&self, other: &Self) -> Self {
        Signal(self.0.chars().filter(|c| other.0.contains(*c)).collect())
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
            .map(|s| Signal(sort_string(s)))
            .collect();
        let output = output
            .trim()
            .split_whitespace()
            .map(|s| Signal(sort_string(s)))
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
}
