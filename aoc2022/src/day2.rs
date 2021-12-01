use crate::input;
use tracing::instrument;

#[derive(Clone, Copy)]
enum GameResult {
    Win,
    Draw,
    Lose,
}
use std::str::FromStr;

use GameResult::*;

impl FromStr for GameResult {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("no GameResult: {s}"),
        })
    }
}

impl GameResult {
    fn score(&self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

use Rps::*;

impl Rps {
    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
    fn win_against(self, other: Rps) -> GameResult {
        match (self, other) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Lose,
        }
    }
}
impl FromStr for Rps {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("no RPS: {s}"),
        })
    }
}

#[instrument(ret)]
fn score(you: Rps, other: Rps) -> u32 {
    you.score() + you.win_against(other).score()
}

fn required_move(other: Rps, result: GameResult) -> Rps {
    match (other, result) {
        (Rock, Win) | (Paper, Draw) | (Scissors, Lose) => Paper,
        (Rock, Draw) | (Paper, Lose) | (Scissors, Win) => Rock,
        (Paper, Win) | (Rock, Lose) | (Scissors, Draw) => Scissors,
    }
}

pub fn solve() -> (u32, u32) {
    let input = input(2);
    (part1(&input), part2(&input))
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_line(line, |s| s.parse::<Rps>().unwrap()))
        .map(|(other, you)| score(you, other))
        .sum()
}
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_line(line, |s| s.parse::<GameResult>().unwrap()))
        .map(|(other, result)| score(required_move(other, result), other))
        .sum()
}

fn parse_line<O>(line: &str, second_parse_fn: impl Fn(&str) -> O) -> (Rps, O) {
    let (other, you) = line.split_once(' ').unwrap();
    (other.parse().unwrap(), second_parse_fn(you))
}

#[test]
fn example() {
    crate::init();
    let input = "A Y
B X
C Z";
    assert_eq!(part1(input), 15);
}

#[test]
fn day2() {
    assert_eq!(solve(), (13924, 13448));
}
