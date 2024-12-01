use std::collections::HashSet;

use crate::input;

pub fn solve() -> (usize, usize) {
    let input = input(6);
    (part1(&input), part2(&input))
}

fn part1(input: &str) -> usize {
    distinct_window_pos(input.as_bytes(), 4)
}

fn part2(input: &str) -> usize {
    distinct_window_pos(input.as_bytes(), 14)
}

fn distinct_window_pos(s: &[u8], window_size: usize) -> usize {
    let indexed = s.iter().enumerate().collect::<Vec<_>>();
    let distinct_window = indexed
        .windows(window_size)
        .find(|window| window.iter().map(|(_, x)| x).collect::<HashSet<_>>().len() == window_size)
        .unwrap();
    // 1-indexed
    distinct_window.last().unwrap().0 + 1
}

#[test]
fn day6() {
    assert_eq!(solve(), (1816, 2625))
}
