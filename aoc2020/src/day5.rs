use std::collections::BTreeSet;

use eyre::{bail, eyre, Result};

const INPUT: &str = include_str!("../input5.txt");

pub fn solve() -> eyre::Result<()> {
    println!("day5.1: {}", solve1(INPUT)?);
    println!("day5.2: {}", solve2(INPUT)?);
    Ok(())
}

fn solve1(input: &str) -> eyre::Result<u32> {
    input
        .lines()
        .map(seat_num)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .max()
        .ok_or_else(|| eyre!("No passes provided"))
        .map(|x| *x)
}

fn solve2(input: &str) -> eyre::Result<u32> {
    let seats = input
        .lines()
        .map(seat_num)
        .collect::<Result<BTreeSet<_>>>()?;
    if seats.is_empty() {
        bail!("No passes provided");
    }

    for i in *(seats.iter().min().unwrap())..*(seats.iter().max().unwrap()) {
        if !seats.contains(&i) && seats.contains(&(i - 1)) && seats.contains(&(i + 1)) {
            return Ok(i);
        }
    }
    bail!("No valid candidate found")
}

fn seat_num(pass: &str) -> eyre::Result<u32> {
    let mut row_min = 0;
    let mut row_max = 127;
    let mut seat_min = 0;
    let mut seat_max = 7;
    for (c, jump) in pass.chars().zip([64, 32, 16, 8, 4, 2, 1, 4, 2, 1].iter()) {
        match c {
            'F' => row_max -= jump,
            'B' => row_min += jump,
            'R' => seat_min += jump,
            'L' => seat_max -= jump,
            _ => bail!("Invalid char {} in line {}", c, pass),
        }
    }
    check_eq(row_min, row_max, "row", pass)?;
    check_eq(seat_min, seat_max, "seat", pass)?;
    Ok(row_max * 8 + seat_max)
}

fn check_eq(min: u32, max: u32, name: &str, pass: &str) -> eyre::Result<()> {
    if max != min {
        bail!(
            "Unspecified {} in pass {} (seat_min: {}, seat_max: {})",
            name,
            pass,
            min,
            max
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5() {
        assert_eq!(solve1(INPUT).unwrap(), 896);
    }

    #[test]
    fn pt1_ex() {
        assert_eq!(solve1("FBFBBFFRLR").unwrap(), 357);
        assert_eq!(solve1("BFFFBBFRRR").unwrap(), 567);
        assert_eq!(solve1("FFFBBBFRRR").unwrap(), 119);
        assert_eq!(solve1("BBFFBBFRLL").unwrap(), 820);
    }
}
