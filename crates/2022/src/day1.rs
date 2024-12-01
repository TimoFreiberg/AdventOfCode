use crate::input;

pub fn solve() -> (u32, u32) {
    let input = input(1);
    (part1(&input), part2(&input))
}

fn part1(input: &str) -> u32 {
    inventories_sorted(input)[0]
}

fn part2(input: &str) -> u32 {
    inventories_sorted(input)[..3].iter().sum()
}

fn inventories_sorted(input: &str) -> Vec<u32> {
    let mut inventories = Vec::new();
    input.lines().fold(0, |calories_so_far, food| {
        let food = food.trim();
        if food.is_empty() {
            inventories.push(calories_so_far);
            0
        } else {
            calories_so_far + food.parse::<u32>().unwrap()
        }
    });
    inventories.sort_unstable();
    inventories.reverse();
    inventories
}

#[test]
fn example() {
    let input = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
    assert_eq!(part1(input), 24000);
}

#[test]
fn day1() {
    assert_eq!(solve(), (74711, 209481));
}
