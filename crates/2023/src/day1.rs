use crate::input;

pub fn solve() -> (u64, u64) {
    let input = input(1);
    let part1 = find_digits(&input, translate_digit);
    let part2 = find_digits(&input, translate_both);

    (part1, part2)
}

fn find_digits(input: &str, find_digit_fn: impl Fn(&str) -> Option<u8>) -> u64 {
    input
        .lines()
        .map(|mut line| {
            let mut digits = Vec::new();
            while !line.is_empty() {
                digits.extend(find_digit_fn(line));
                line = &line[1..];
            }
            format!("{}{}", digits[0], digits.last().unwrap())
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}
fn translate_both(s: &str) -> Option<u8> {
    translate_digit(s).or_else(|| translate_word(s))
}

fn translate_word(s: &str) -> Option<u8> {
    for (word, value) in [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ] {
        if s.starts_with(word) {
            return Some(value);
        }
    }
    None
}

fn translate_digit(s: &str) -> Option<u8> {
    s[..1].parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap() {
        let result = find_digits(
            "twone
sevenine",
            translate_both,
        );
        assert_eq!(result, 100);
    }

    #[test]
    fn day1() {
        tracing_subscriber::fmt::try_init().ok();
        assert_eq!(solve(), (55002, 55093))
    }

    #[test]
    fn example() {
        tracing_subscriber::fmt::try_init().ok();
        let result = find_digits(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
            translate_both,
        );
        assert_eq!(result, 281);
    }
}
