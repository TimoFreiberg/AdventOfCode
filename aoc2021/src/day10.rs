use std::collections::VecDeque;

use eyre::Result;
use tracing::debug;

use crate::input;

pub fn solve() -> Result<(u32, u64)> {
    let input = input(10);
    let lines = parse(&input);
    let analyses = lines.into_iter().map(Line::analyze).collect::<Vec<_>>();

    Ok((part1(&analyses), part2(&analyses)))
}

fn part1(analyses: &[LineAnalysis]) -> u32 {
    analyses
        .iter()
        .filter_map(LineAnalysis::score_corrupt)
        .sum()
}

fn part2(analyses: &[LineAnalysis]) -> u64 {
    let mut scores = analyses
        .iter()
        .filter_map(LineAnalysis::score_incomplete)
        .collect::<Vec<_>>();
    scores.sort_unstable();
    let result = scores[(scores.len() / 2)];
    debug!("result: {} in len {}: {:?}", result, scores.len(), scores);
    result
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| Line {
            data: l.chars().collect(),
        })
        .collect()
}

#[derive(Debug)]
struct Line {
    data: Vec<char>,
}

impl Line {
    fn analyze(self) -> LineAnalysis {
        let mut state = VecDeque::with_capacity(10);
        for (ix, c) in self.data.iter().enumerate() {
            let (dir, delimiter) = parse_char(*c);
            match dir {
                Direction::Open => {
                    state.push_front(delimiter);
                }
                Direction::Close => match state.front() {
                    Some(d) if *d == delimiter => {
                        state.pop_front();
                    }
                    _ => {
                        debug!(
                            "line corrupted at {} with {}:\n{}\n{:>width$}^\nstate: {:?}\n\n",
                            ix,
                            c,
                            String::from_iter(&self.data),
                            ' ',
                            state,
                            width = ix
                        );
                        return LineAnalysis::Corrupted { next_char: *c };
                    }
                },
            }
        }
        if state.is_empty() {
            LineAnalysis::Ok {}
        } else {
            LineAnalysis::Incomplete { leftovers: state }
        }
    }
}

#[derive(Debug)]
enum LineAnalysis {
    Ok {},
    Incomplete { leftovers: VecDeque<Delimiter> },
    Corrupted { next_char: char },
}

impl LineAnalysis {
    fn score_corrupt(&self) -> Option<u32> {
        Some(match self {
            LineAnalysis::Ok { .. } => return None,
            LineAnalysis::Incomplete { .. } => return None,
            LineAnalysis::Corrupted { next_char, .. } => match next_char {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Invalid illegal character {:?} in {:?}", next_char, self),
            },
        })
    }
    fn score_incomplete(&self) -> Option<u64> {
        let leftovers = match self {
            LineAnalysis::Ok {} => return None,
            LineAnalysis::Incomplete { leftovers } => leftovers,
            LineAnalysis::Corrupted { .. } => return None,
        };
        let mut score = 0;
        for leftover in leftovers {
            score *= 5;
            score += match leftover {
                Delimiter::Paren => 1,
                Delimiter::Bracket => 2,
                Delimiter::Brace => 3,
                Delimiter::Angle => 4,
            };
        }
        debug!("Score {} for {:?}", score, self);
        Some(score)
    }
}

fn parse_char(c: char) -> (Direction, Delimiter) {
    match c {
        '(' => (Direction::Open, Delimiter::Paren),
        ')' => (Direction::Close, Delimiter::Paren),
        '[' => (Direction::Open, Delimiter::Bracket),
        ']' => (Direction::Close, Delimiter::Bracket),
        '{' => (Direction::Open, Delimiter::Brace),
        '}' => (Direction::Close, Delimiter::Brace),
        '<' => (Direction::Open, Delimiter::Angle),
        '>' => (Direction::Close, Delimiter::Angle),

        _ => panic!("Invalid char {:?}", c),
    }
}

enum Direction {
    Open,
    Close,
}

#[derive(PartialEq, Eq)]
enum Delimiter {
    Paren,
    Bracket,
    Brace,
    Angle,
}

impl std::fmt::Debug for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Paren => write!(f, "')'"),
            Self::Bracket => write!(f, "']'"),
            Self::Brace => write!(f, "'}}'"),
            Self::Angle => write!(f, "'>'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10() {
        let (part1, part2) = solve().unwrap();
        assert_eq!(part1, 168417);
        assert!(part2 > 964715401);
        assert!(part2 > 969511484);
        assert_eq!(part2, 2802519786);
    }

    #[test]
    fn day10_ex() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let lines = parse(input);
        let analyses = lines.into_iter().map(Line::analyze).collect::<Vec<_>>();
        let part1 = part1(&analyses);
        let part2 = part2(&analyses);
        assert_eq!(part1, 26397);
        assert_eq!(part2, 288957);
    }

    #[test]
    fn incomplete_score() {
        let incomplete = LineAnalysis::Incomplete {
            leftovers: VecDeque::from_iter(
                [
                    ']', '>', ')', '}', '>', ')', ')', '>', ')', ')', ']', '>', ']', '>', ')',
                ]
                .into_iter()
                .map(|c| parse_char(c).1),
            ),
        };
        let score = incomplete.score_incomplete().unwrap();
        assert_eq!(score, 17522208071);
    }
}
