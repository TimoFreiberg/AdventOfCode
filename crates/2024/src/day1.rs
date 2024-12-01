use std::collections::HashMap;

use crate::input;

pub fn solve() -> eyre::Result<(u64, u64)> {
    let input = input(1);
    let lists = Lists::parse(&input)?;
    let part1 = part1(&lists)?;
    let part2 = part2(&lists);

    // let part2 = find_digits(&input, translate_both);

    Ok((part1, part2))
}

fn part1(lists: &Lists) -> eyre::Result<u64> {
    Ok(lists
        .left
        .iter()
        .zip(&lists.right)
        .map(|(l, r)| ((l - r).abs()) as u64)
        .sum())
}

fn part2(lists: &Lists) -> u64 {
    let mut occurrences_in_right = HashMap::new();
    for right_item in &lists.right {
        *occurrences_in_right.entry(*right_item).or_insert(0) += 1;
    }

    lists
        .left
        .iter()
        .map(|l| (*l as u64) * occurrences_in_right.get(l).copied().unwrap_or(0))
        .sum()
}

struct Lists {
    left: Vec<i64>,
    right: Vec<i64>,
}

impl Lists {
    fn parse(s: &str) -> eyre::Result<Self> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for line in s.lines() {
            let mut pairs = line.split_whitespace();
            left.push(pairs.next().unwrap().parse::<i64>()?);
            right.push(pairs.next().unwrap().parse::<i64>()?);
        }

        left.sort_unstable();
        right.sort_unstable();
        Ok(Self { left, right })
    }
}
