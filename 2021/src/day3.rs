use std::collections::BTreeMap;

use eyre::Result;

use crate::input;

pub fn solve() -> Result<(u32, u32)> {
    let input = input(3);
    solve_for_input(&input)
}

fn solve_for_input(input: &str) -> Result<(u32, u32), eyre::Error> {
    let lines = parse(input);

    let part1 = part1(&lines);

    let part2 = part2(&lines);

    Ok((part1, part2))
}

fn part1(lines: &[Vec<char>]) -> u32 {
    let (most_common_bits, least_common_bits) = bits(lines);
    let gamma = u32::from_str_radix(&most_common_bits, 2).unwrap();
    let epsilon = u32::from_str_radix(&least_common_bits, 2).unwrap();
    gamma * epsilon
}

fn part2(lines: &[Vec<char>]) -> u32 {
    let (mut most_common_bits, mut least_common_bits) = bits(lines);
    let line_len = most_common_bits.len();

    let mut oxygen_input = lines.to_vec();
    for pos in 0..line_len {
        oxygen_input.retain(|line| line[pos] == most_common_bits.chars().nth(pos).unwrap());
        most_common_bits = bits(&oxygen_input).0;
        if oxygen_input.len() == 1 {
            break;
        }
    }
    assert_eq!(oxygen_input.len(), 1);
    let oxygen_rating =
        u32::from_str_radix(&String::from_iter(oxygen_input.get(0).unwrap()), 2).unwrap();

    let mut co2_scrubber_input = lines.to_vec();
    for pos in 0..line_len {
        co2_scrubber_input.retain(|line| {
            line[pos]
                == least_common_bits
                    .chars()
                    .nth(pos)
                    .unwrap_or_else(|| panic!("Less than {} in {:?}", pos, least_common_bits))
        });
        least_common_bits = bits(&co2_scrubber_input).1;
        if co2_scrubber_input.len() == 1 {
            break;
        }
    }
    assert_eq!(co2_scrubber_input.len(), 1);
    let co2_scrubber_rating =
        u32::from_str_radix(&String::from_iter(co2_scrubber_input.get(0).unwrap()), 2).unwrap();

    oxygen_rating * co2_scrubber_rating
}

fn bits(lines: &[Vec<char>]) -> (String, String) {
    let mut ones: BTreeMap<usize, u32> = BTreeMap::new();
    let mut line_count = 0;
    for line in lines {
        line_count += 1;
        for (ix, c) in line.iter().enumerate() {
            if *c == '1' {
                *ones.entry(ix).or_insert(0) += 1;
            } else {
                ones.entry(ix).or_insert(0);
            }
        }
    }

    let most_common_bits = most_common_bits(&ones, line_count as f32 / 2.0);
    let least_common_bits = least_common_bits(&ones, line_count as f32 / 2.0);

    (most_common_bits, least_common_bits)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    lines
}

fn most_common_bits(ones: &BTreeMap<usize, u32>, mid_point: f32) -> String {
    let result = ones
        .values()
        .map(|one_count| {
            if *one_count as f32 >= mid_point {
                '1'
            } else {
                '0'
            }
        })
        .collect();
    result
}

fn least_common_bits(ones: &BTreeMap<usize, u32>, mid_point: f32) -> String {
    ones.values()
        .map(|one_count| {
            if *one_count as f32 >= mid_point {
                '0'
            } else {
                '1'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3() {
        assert_eq!(solve().unwrap(), (4118544, 3832770));
    }

    #[test]
    fn day3_pt2_example() {
        assert_eq!(
            solve_for_input(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            )
            .unwrap()
            .1,
            230
        );
    }
}
