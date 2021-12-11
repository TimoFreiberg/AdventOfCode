use log::debug;
use std::str::pattern::Pattern;

const INPUT: &str = include_str!("../input2.txt");

pub fn solve() -> eyre::Result<()> {
    println!("day2.1: {}", solve1(INPUT));
    println!("day2.2: {}", solve2(INPUT));
    Ok(())
}

fn solve1(input: &str) -> String {
    input
        .lines()
        .filter_map(|it| {
            let valid = PwLine::parse(it);
            debug!("parsed pw: {:?}", valid);
            valid
        })
        .filter(|it| it.matches_1())
        .count()
        .to_string()
}

fn solve2(input: &str) -> String {
    input
        .lines()
        .filter_map(|it| {
            let valid = PwLine::parse(it);
            debug!("parsed pw: {:?}", valid);
            valid
        })
        .filter(|it| it.matches_2())
        .count()
        .to_string()
}

#[derive(Debug)]
struct PwLine<'a> {
    pw: &'a str,
    lower: usize,
    upper: usize,
    which_char: char,
}

impl<'a> PwLine<'a> {
    fn parse(line: &'a str) -> Option<Self> {
        let (num1, line) = parse(line, '-')?;
        let line = &line[1..];
        let (num2, line) = parse(line, ' ')?;
        let line = &line[1..];
        let (which_char, line) = line.split_at(1);
        let pw = &line[2..];

        let lower = num1.parse::<usize>().ok()?;
        let upper = num2.parse::<usize>().ok()?;
        let which_char = which_char.chars().next()?;
        Some(PwLine {
            pw,
            lower,
            upper,
            which_char,
        })
    }
    fn matches_1(&self) -> bool {
        debug!(
            "pw {:?} must have {}-{} occurences of {:?}",
            self.pw, self.lower, self.upper, self.which_char
        );
        let char_count = self.pw.chars().filter(|c| *c == self.which_char).count();
        char_count >= self.lower && char_count <= self.upper
    }
    fn matches_2(&self) -> bool {
        let lower = self.lower - 1;
        let upper = self.upper - 1;
        if upper >= self.pw.len() {
            return false;
        }
        let char1 = self.pw.chars().nth(lower).unwrap();
        let char2 = self.pw.chars().nth(upper).unwrap();
        debug!(
            "pw {:?} must have {:?} at pos {} XOR at pos {} ({:?}/{:?})",
            self.pw,
            self.which_char,
            lower,
            upper,
            self.pw.chars().nth(lower),
            self.pw.chars().nth(upper),
        );
        (self.which_char == char1) ^ (self.which_char == char2)
    }
}

fn parse<'a, P: Pattern<'a>>(input: &'a str, pat: P) -> Option<(&'a str, &'a str)> {
    let ix = input.find(pat)?;
    Some(input.split_at(ix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_ex() {
        assert_eq!(solve1("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc",), "2");
    }

    #[test]
    fn pt2_ex() {
        assert_eq!(solve2("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc",), "1");
    }

    #[test]
    fn day2() {
        assert_eq!(solve1(INPUT), "638");
        assert_eq!(solve2(INPUT), "699");
    }
}
