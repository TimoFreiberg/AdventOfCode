use std::{iter, ops::RangeInclusive};

use eyre::Result;

pub fn solve() -> Result<(i32, usize)> {
    let target_x = 211..=232;
    let target_y = -124..=-69;

    Ok((part1(target_y.clone()), part2(target_x, target_y)))
}

fn part1(target_y: RangeInclusive<i32>) -> i32 {
    let y = (max_abs(&target_y)) - 1;
    y_trajectory(y, target_y).max().unwrap()
}

fn max_abs(range: &RangeInclusive<i32>) -> i32 {
    range.end().abs().max(range.start().abs())
}

fn part2(target_x: RangeInclusive<i32>, target_y: RangeInclusive<i32>) -> usize {
    let mut count = 0;
    for x_vel in 0..=*target_x.end() {
        let max_abs = max_abs(&target_y);
        for y_vel in -max_abs..=max_abs {
            if x_trajectory(x_vel, target_x.clone())
                .zip(y_trajectory(y_vel, target_y.clone()))
                .any(|(x, y)| target_x.contains(&x) && target_y.contains(&y))
            {
                count += 1;
            }
        }
    }
    count
}

fn x_trajectory(mut x: i32, target_x: RangeInclusive<i32>) -> impl Iterator<Item = i32> {
    let mut pos = 0;
    iter::from_fn(move || {
        if pos > *target_x.end() {
            return None;
        }
        pos += x;
        x = (x - 1).max(0);
        Some(pos)
    })
}
fn y_trajectory(mut y: i32, target_y: RangeInclusive<i32>) -> impl Iterator<Item = i32> {
    let mut pos = 0;
    iter::from_fn(move || {
        if pos < -max_abs(&target_y) {
            return None;
        }
        pos += y;
        y -= 1;
        Some(pos)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part1(-10..=-5), 45);
        assert_eq!(part2(20..=30, -10..=-5), 112);
    }
}
