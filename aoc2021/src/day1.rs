use eyre::Result;

use crate::input;

pub fn solve() -> Result<(String, String)> {
    let input = input(1);
    let depths: Vec<_> = input
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Result<_, _>>()?;

    Ok((part1(&depths), part2(&depths)))
}

fn part2(depths: &[u32]) -> String {
    let windows = depths
        .iter()
        .zip(&depths[1..])
        .zip(&depths[2..])
        .map(|((first, second), third)| first + second + third)
        .collect::<Vec<_>>();
    part1(&windows)
}

fn part1(depths: &[u32]) -> String {
    depths
        .iter()
        .zip(&depths[1..])
        .filter(|(first, second)| first < second)
        .count()
        .to_string()
}
