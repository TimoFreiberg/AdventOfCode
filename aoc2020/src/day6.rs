use std::collections::HashMap;

const INPUT: &str = include_str!("../input6.txt");

pub fn solve() -> eyre::Result<()> {
    println!("day6.1: {}", solve1(INPUT));
    println!("day6.2: {}", solve2(INPUT));
    Ok(())
}

fn solve1(input: &str) -> usize {
    groups(input)
        .map(|group| {
            let m = count_answers(group);
            m.len()
        })
        .sum()
}

fn solve2(input: &str) -> usize {
    groups(input)
        .map(|group| {
            let m = count_answers(group);
            let group_size = group.lines().count() as u32;
            m.values().filter(|count| **count == group_size).count()
        })
        .sum()
}

fn groups(input: &str) -> impl Iterator<Item = &str> {
    input.split("\n\n")
}

fn count_answers(group: &str) -> HashMap<char, u32> {
    let mut m = HashMap::new();
    for line in group.lines() {
        for c in line.chars().filter(|c| c.is_alphabetic()) {
            *m.entry(c).or_insert(0) += 1
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6() {
        assert_eq!(solve1(INPUT), 6585);
    }

    #[test]
    fn part1_ex() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(solve1(input), 11);
    }

    #[test]
    fn part2_ex() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(solve2(input), 6);
    }
}
