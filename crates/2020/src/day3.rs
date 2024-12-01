use std::iter::successors;

const INPUT: &str = include_str!("../input3.txt");

pub fn solve() -> eyre::Result<()> {
    println!("day3.1: {}", solve1(INPUT));
    println!("day3.2: {}", solve2(INPUT));
    Ok(())
}

fn solve1(input: &str) -> usize {
    let field = parse_field(input);

    count_path_tree_collisions(&field, (1, 3))
}

fn solve2(input: &str) -> usize {
    let field = parse_field(input);
    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|step| count_path_tree_collisions(&field, *step))
        .product()
}

fn parse_field(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn count_path_tree_collisions(field: &[Vec<char>], step: (usize, usize)) -> usize {
    successors(Some((0, 0)), |(y, x)| Some((y + step.0, x + step.1)))
        .take_while(|(y, _)| *y < field.len())
        .map(|(y, x)| {
            let line = &field[y];
            line[x % line.len()]
        })
        .filter(|c| *c == '#')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3() {
        assert_eq!(solve1(INPUT), 292);
        assert_eq!(solve2(INPUT), 9354744432);
    }
}
