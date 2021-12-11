use eyre::eyre;
use itertools::Itertools;

pub fn solve() -> eyre::Result<()> {
    println!("day1.1: {}", solve1()?);
    println!("day1.2: {}", solve2()?);

    Ok(())
}

const INPUT: &str = include_str!("../input1.csv");

fn solve1() -> eyre::Result<i32> {
    let lines = INPUT
        .lines()
        .map(|it| it.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    solve_for_combinations(&lines, 2)
}

fn solve2() -> eyre::Result<i32> {
    let lines = INPUT
        .lines()
        .map(|it| it.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    solve_for_combinations(&lines, 3)
}

fn solve_for_combinations(lines: &[i32], combinations: usize) -> eyre::Result<i32> {
    let result: i32 = lines
        .iter()
        .combinations(combinations)
        .find(|it| it.iter().copied().sum::<i32>() == 2020)
        .ok_or_else(|| eyre!("no pair found"))?
        .iter()
        .copied()
        .product();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(solve1().unwrap(), 988771);
        assert_eq!(solve2().unwrap(), 171933104);
    }
}
