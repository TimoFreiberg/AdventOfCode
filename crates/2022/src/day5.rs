use std::collections::BTreeMap;

use itertools::Itertools;
use regex::Regex;
use tracing::debug;

use crate::input;

pub fn solve() -> (String, String) {
    let input = input(5);
    (part1(&input), part2(&input))
}

fn part1(input: &str) -> String {
    let (mut stack, moves) = parse(input);
    for m in moves {
        stack.apply_single_move(m);
    }
    stack.top_of_the_stack()
}

fn part2(input: &str) -> String {
    let (mut stack, moves) = parse(input);
    for m in moves {
        stack.apply_batch_move(m);
    }
    stack.top_of_the_stack()
}

#[derive(Debug, PartialEq)]
struct Crates(BTreeMap<usize, Vec<char>>);
impl Crates {
    fn apply_single_move(&mut self, m: Move) {
        let before = format!("{self:?}");
        for _ in 0..m.amount {
            let c = self.0.get_mut(&m.from).unwrap().pop().unwrap();
            self.0.get_mut(&m.to).unwrap().push(c);
        }
        debug!(?before, after = ?self);
    }

    fn apply_batch_move(&mut self, m: Move) {
        let before = format!("{self:?}");
        let from = self.0.get_mut(&m.from).unwrap();
        let crates: Vec<_> = from.drain((from.len() - m.amount)..).collect();
        self.0.get_mut(&m.to).unwrap().extend(crates);
        debug!(?before, after = ?self);
    }

    fn top_of_the_stack(&self) -> String {
        let mut s = String::new();
        for stack in self.0.values() {
            s.push(*stack.last().unwrap());
        }
        s
    }
}

fn parse(input: &str) -> (Crates, Vec<Move>) {
    let separator_ix = input.find("\n\n").unwrap();
    let (crates, moves) = input.split_at(separator_ix);
    let moves = &moves[2..];
    (parse_crates(crates), parse_moves(moves))
}

fn parse_crates(input: &str) -> Crates {
    let mut result: BTreeMap<usize, Vec<char>> = BTreeMap::new();
    for line in input.lines().take_while(|l| !l.starts_with(" 1")) {
        for (pos, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            match chunk.nth(1) {
                Some(c) if !c.is_whitespace() => {
                    result.entry(pos + 1).or_default().insert(0, c);
                }
                _ => {}
            }
        }
    }
    Crates(result)
}

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}
fn parse_moves(input: &str) -> Vec<Move> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let get_num = |pos| captures.get(pos).unwrap().as_str().parse().unwrap();
            Move {
                amount: get_num(1),
                from: get_num(2),
                to: get_num(3),
            }
        })
        .collect()
}

#[test]
fn parses() {
    crate::init();
    let input = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

";
    assert_eq!(
        parse_crates(input),
        Crates(BTreeMap::from_iter([
            (1, vec!['Z', 'N']),
            (2, vec!['M', 'C', 'D']),
            (3, vec!['P'])
        ]))
    )
}

#[test]
fn day5() {
    assert_eq!(solve(), ("LJSVLTWQM".into(), "BRQWDBBJM".into()))
}
