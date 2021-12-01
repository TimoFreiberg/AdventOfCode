use std::collections::HashSet;

use tap::Pipe;

use crate::input;

pub fn solve() -> (u32, u32) {
    let input = input(3);
    (part1(&input), part2(&input))
}

fn part1(input: &str) -> u32 {
    parse_compartments(input)
        .into_iter()
        .map(|it| duplicate_item(it).pipe(priority))
        .sum()
}

fn part2(input: &str) -> u32 {
    parse_group(input)
        .into_iter()
        .map(|it| duplicate_item(it).pipe(priority))
        .sum()
}

fn parse_compartments(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let mid = line.len() / 2;
            vec![&line[..mid], &line[mid..]]
        })
        .collect()
}

fn parse_group(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .array_chunks()
        .map(|[l1, l2, l3]| vec![l1, l2, l3])
        .collect()
}

fn duplicate_item(parts: Vec<&str>) -> char {
    let mut parts = parts.into_iter();
    let mut duplicates = parts.next().unwrap().chars().collect::<HashSet<_>>();
    for part in parts {
        let part_chars = part.chars().collect();
        duplicates = duplicates.intersection(&part_chars).copied().collect();
    }
    assert_eq!(duplicates.len(), 1);
    duplicates.into_iter().next().unwrap()
}

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32) - 38
    } else {
        (c as u32) - 96
    }
}

#[test]
fn day3() {
    assert_eq!(solve(), (8123, 2620))
}
