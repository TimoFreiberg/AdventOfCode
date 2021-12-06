use eyre::{Context, Result};

use crate::input;

pub fn solve() -> Result<(u128, u128)> {
    let input = input(6);
    let mut fishies = parse(&input)?;

    for _ in 0..80 {
        fishies.step();
    }
    let part1 = fishies.sum();

    for _ in 0..(256 - 80) {
        fishies.step();
    }
    let part2 = fishies.sum();

    Ok((part1, part2))
}

struct FishBuckets([u128; 9]);

impl FishBuckets {
    fn step(&mut self) {
        let bucket = &mut self.0;
        let spawning = bucket[0];
        for n in 0..8 {
            bucket[n] = bucket[n + 1];
        }
        bucket[6] += spawning;
        bucket[8] = spawning;
    }
    fn sum(&self) -> u128 {
        self.0.iter().sum()
    }
}

fn parse(input: &str) -> Result<FishBuckets> {
    let mut result = FishBuckets([0; 9]);

    for s in input.trim().split(',') {
        let x = s
            .parse::<usize>()
            .with_context(|| format!("Invalid number: {:?}", s))?;
        result.0[x] += 1;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6() {
        assert_eq!(solve().unwrap(), (377263, 1695929023803))
    }

    #[test]
    fn day6_ex() {
        let input = "3,4,3,1,2";
        let mut fishies = parse(input).unwrap();
        assert_eq!(fishies.0, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
        fishies.step();
        assert_eq!(fishies.0, [1, 1, 2, 1, 0, 0, 0, 0, 0]);
        fishies.step();
        assert_eq!(fishies.0, [1, 2, 1, 0, 0, 0, 1, 0, 1]);
        fishies.step();
        assert_eq!(fishies.0, [2, 1, 0, 0, 0, 1, 1, 1, 1]);
    }
}
