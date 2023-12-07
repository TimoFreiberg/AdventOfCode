use std::ops::RangeInclusive;

use tracing::instrument;

use crate::input;

pub fn solve() -> (u32, u32) {
    let input = input(4);

    (part1(&input), part2(&input))
}

fn part1(input: &str) -> u32 {
    let ranges = parse(input);
    ranges
        .into_iter()
        .filter(|(l, r)| fully_contains(l, r))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let ranges = parse(input);
    ranges.into_iter().filter(|(l, r)| overlaps(l, r)).count() as u32
}

fn fully_contains(left: &Range, right: &Range) -> bool {
    let f = |greater: &Range, smaller: &Range| {
        greater.start() <= smaller.start() && greater.end() >= smaller.end()
    };
    f(left, right) || f(right, left)
}

#[instrument(ret)]
fn overlaps(left: &Range, right: &Range) -> bool {
    let f = |l: &Range, r: &Range| l.start() <= r.start() && l.end() >= r.start();
    f(left, right) || f(right, left)
}

type Range = RangeInclusive<i32>;

fn parse(input: &str) -> Vec<(Range, Range)> {
    fn parse_range(range: &str) -> Range {
        let (from, to) = range.split_once('-').unwrap();
        from.parse().unwrap()..=to.parse().unwrap()
    }
    input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(',').unwrap();
            (parse_range(l), parse_range(r))
        })
        .collect()
}

#[test]
fn example() {
    crate::init();
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(part2(input), 4);
}

#[test]
fn day4() {
    assert_eq!(solve(), (576, 905));
}
