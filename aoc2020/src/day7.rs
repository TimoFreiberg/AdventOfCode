use std::collections::{HashMap, HashSet};

use eyre::{eyre, Context};
use log::debug;

const INPUT: &str = include_str!("../input7.txt");

pub fn solve() -> eyre::Result<()> {
    println!("day7.1: {}", solve1(INPUT)?);
    println!("day7.2: {}", solve2(INPUT)?);
    Ok(())
}

fn solve1(input: &str) -> eyre::Result<usize> {
    let db = BagDb::parse(input)?;

    Ok(db.possible_containers(&("shiny".to_string(), "gold".to_string())))
}

fn solve2(input: &str) -> eyre::Result<usize> {
    let db = BagDb::parse(input)?;

    Ok(db.contained(&("shiny".to_string(), "gold".to_string())))
}

struct BagDb {
    contains: HashMap<BagKey, Vec<(usize, BagKey)>>,
    contained_in: HashMap<BagKey, Vec<BagKey>>,
}

impl BagDb {
    fn parse(lines: &str) -> eyre::Result<Self> {
        let mut contains = HashMap::new();
        let mut contained_in = HashMap::new();
        for line in lines.lines() {
            let (bag, bag_contains) = Bag::parse(line)?;
            let k = bag.keys();
            for (_, k2) in &bag_contains {
                contained_in
                    .entry(k2.clone())
                    .or_insert_with(Vec::new)
                    .push(k.clone());
            }
            contains.insert(k, bag_contains);
        }
        Ok(Self {
            contains,
            contained_in,
        })
    }
    fn possible_containers(&self, k: &BagKey) -> usize {
        let mut next = match self.contained_in.get(k) {
            Some(it) => it.iter().collect::<Vec<_>>(),
            None => return 0,
        };
        let mut containers = HashSet::new();
        while !next.is_empty() {
            let mut next2 = Vec::new();
            for (modifier, color) in next {
                let k2 = (modifier.clone(), color.clone());
                if let Some(bag) = self.contained_in.get(&k2) {
                    next2.extend(bag);
                }
                containers.insert(k2);
            }
            next = next2;
        }
        containers.len()
    }
    fn contained(&self, k: &BagKey) -> usize {
        let contains = match self.contains.get(k) {
            Some(it) => it,
            None => {
                debug!("{:?} doesn't contain any bags", k);
                return 0;
            }
        };
        let res = contains
            .iter()
            .map(|(x, k_inner)| {
                let res = x * (1 + self.contained(k_inner));
                debug!("Counting {:?} inside {:?}: {}", k_inner, k, res);
                res
            })
            .sum();
        debug!("{:?} contains {}", k, res);
        res
    }
}

type BagKey = (String, String);

#[derive(Debug)]
struct Bag {
    modifier: String,
    color: String,
}

impl Bag {
    fn parse(s: &str) -> eyre::Result<(Self, Vec<(usize, BagKey)>)> {
        (|| -> eyre::Result<_> {
            let mut words = s.split_ascii_whitespace();
            let modifier = words
                .next()
                .ok_or_else(|| eyre!("Missing modifier"))?
                .to_string();
            let color = words
                .next()
                .ok_or_else(|| eyre!("Missing color"))?
                .to_string();
            let contains = words.skip(2).collect::<Vec<_>>();

            let contains = if contains.first() == Some(&"no") {
                Vec::new()
            } else {
                contains
                    .chunks(4)
                    .map(|bag| {
                        Ok((
                            bag[0].parse::<usize>()?,
                            (bag[1].to_string(), bag[2].to_string()),
                        ))
                    })
                    .collect::<eyre::Result<Vec<_>>>()?
            };
            Ok((Bag { color, modifier }, contains))
        })()
        .with_context(|| format!("Parsing {:?}", s))
    }
    fn keys(&self) -> (String, String) {
        (self.modifier.clone(), self.color.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        assert_eq!(solve1(INPUT).unwrap(), 179);
        assert_eq!(solve2(INPUT).unwrap(), 18925);
    }

    #[test]
    fn part1_ex() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(solve1(input).unwrap(), 4);
    }

    #[test]
    fn part2_ex1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
        assert_eq!(solve2(input).unwrap(), 32);
    }

    #[test]
    fn part2_ex2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";
        assert_eq!(solve2(input).unwrap(), 126);
    }
}
