use eyre::Result;

use crate::input;

pub fn solve() -> Result<(usize, usize)> {
    let input = input(7);
    let positions = parse(&input)?;

    let part1 = search_best(&positions, fuel_cost_pt1);
    let part2 = search_best(&positions, fuel_cost_pt2);

    Ok((part1, part2))
}

fn search_best(positions: &[u16], fuel_cost_fn: impl Fn(&[u16], usize) -> usize) -> usize {
    let mean_pos = positions.iter().map(|x| *x as usize).sum::<usize>() / positions.len();
    let cost = fuel_cost_fn(positions, mean_pos);

    let best_smaller = best_in_direction(positions, |x| x - 1, &fuel_cost_fn, mean_pos, cost);
    let best_larger = best_in_direction(positions, |x| x + 1, fuel_cost_fn, mean_pos, cost);
    best_smaller.min(best_larger)
}

fn best_in_direction(
    positions: &[u16],
    direction: impl Fn(usize) -> usize,
    fuel_cost_fn: impl Fn(&[u16], usize) -> usize,
    start: usize,
    start_fuel_cost: usize,
) -> usize {
    if start == 0 {
        return start;
    }
    let next = direction(start);
    let next_fuel_cost = fuel_cost_fn(positions, next);
    if next_fuel_cost <= start_fuel_cost {
        best_in_direction(positions, direction, fuel_cost_fn, next, next_fuel_cost)
    } else {
        start_fuel_cost
    }
}

fn fuel_cost_pt1(positions: &[u16], to: usize) -> usize {
    let mut result = 0;
    for from in positions {
        result += abs_diff(*from, to);
    }
    result
}

fn fuel_cost_pt2(positions: &[u16], to: usize) -> usize {
    let mut result = 0;
    for from in positions {
        let distance = abs_diff(*from, to);
        let cost: usize = (1..).take(distance).sum();
        result += cost;
    }
    result
}

fn abs_diff(x: u16, y: usize) -> usize {
    let x = usize::from(x);
    if x > y {
        x - y
    } else {
        y - x
    }
}

fn parse(input: &str) -> Result<Vec<u16>> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().map_err(Into::into))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        assert_eq!(solve().unwrap(), (347509, 98257206));
    }
}
