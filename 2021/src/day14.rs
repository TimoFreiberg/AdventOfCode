use std::collections::BTreeMap;

use eyre::Result;

use crate::input;

pub fn solve() -> Result<(u64, u64)> {
    let input = input(14);

    let (template, rules) = parse(&input);

    Ok((run(template.clone(), &rules, 10), run(template, &rules, 40)))
}

fn run(polymer: Vec<u8>, rules: &Rules, iterations: u8) -> u64 {
    let last_char = *polymer.last().unwrap();
    let mut buckets = buckets(polymer);
    for _i in 0..iterations {
        buckets = rules.step(buckets);
    }

    let mut occurrences = BTreeMap::new();
    for ((l, _), count) in buckets {
        *occurrences.entry(l).or_insert(0) += count;
    }
    *occurrences.get_mut(&last_char).unwrap() += 1;
    let (_, least_common) = occurrences.iter().min_by_key(|(_c, count)| *count).unwrap();
    let (_, most_common) = occurrences.iter().max_by_key(|(_c, count)| *count).unwrap();
    most_common - least_common
}

fn buckets(polymer: Vec<u8>) -> BTreeMap<(u8, u8), u64> {
    let mut buckets = BTreeMap::new();
    for (l, r) in polymer.iter().zip(&polymer[1..]) {
        *buckets.entry((*l, *r)).or_default() += 1;
    }
    buckets
}

fn parse(input: &str) -> (Vec<u8>, Rules) {
    let mut lines = input.lines();
    let template = lines.next().unwrap();
    lines.next();
    let mut rules = BTreeMap::new();
    for line in lines {
        let (pattern, produces) = line.split_once(" -> ").unwrap();
        let bytes = pattern.as_bytes();
        rules.insert((bytes[0], bytes[1]), produces.as_bytes()[0]);
    }

    (template.as_bytes().to_owned(), Rules(rules))
}

struct Rules(BTreeMap<(u8, u8), u8>);

impl Rules {
    fn step(&self, polymer: BTreeMap<(u8, u8), u64>) -> BTreeMap<(u8, u8), u64> {
        let mut result = BTreeMap::new();
        for ((l, r), count) in polymer {
            match self.0.get(&(l, r)) {
                Some(&mid) => {
                    *result.entry((l, mid)).or_default() += count;
                    *result.entry((mid, r)).or_default() += count;
                }
                None => {
                    *result.entry((l, r)).or_default() += count;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14() {
        let (part1, part2) = solve().unwrap();

        assert_eq!(part1, 3143);
        assert_eq!(part2, 4110215602456);
    }

    #[test]
    fn day14_ex() {
        let (template, rules) = parse(
            "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
        );
        assert_eq!(run(template, &rules, 10), 1588);
    }
}
