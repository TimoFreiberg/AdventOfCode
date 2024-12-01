use std::str::FromStr;

use eyre::{eyre, Result};

use crate::input;

pub fn solve() -> (u64, u64) {
    let input = input(4);
    let cards = parse(&input);
    (part1(&cards), part2(&cards))
}

fn part1(cards: &[Card]) -> u64 {
    cards.iter().map(|card| card.worth()).sum()
}
fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>>>()
        .unwrap()
}

fn part2(cards: &[Card]) -> u64 {
    let mut cards_count = cards.iter().map(|card| (1, card)).collect::<Vec<_>>();
    let mut total_count = 0;
    for ix in 0..cards_count.len() {
        let (count, card) = cards_count[ix];
        tracing::debug!(%count, ?card);
        total_count += count;
        let card_matches = card.matches();
        for ix2 in (ix + 1)..(ix + 1 + card_matches as usize) {
            tracing::debug!("copying card {ix2}");
            if ix2 >= cards_count.len() {
                break;
            }
            cards_count[ix2].0 += count;
        }
    }
    total_count
}

#[derive(Debug)]
struct Card {
    number: u64,
    winning_numbers: Vec<u64>,
    have_numbers: Vec<u64>,
}
impl Card {
    fn matches(&self) -> u64 {
        self.have_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u64
    }
    fn worth(&self) -> u64 {
        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            let result = 2u64.pow((matches - 1) as u32);
            tracing::debug!(num = %self.number, %matches, %result);
            result
        }
    }
}
impl FromStr for Card {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s;
        let s = s.strip_prefix("Card").ok_or(eyre!("Card prefix"))?;
        let (num, rest) = s.trim().split_once(':').ok_or(eyre!("no :"))?;
        let number = num.parse().map_err(|e| {
            tracing::warn!(?line, ?num, %e, "parse err");
            e
        })?;
        let (winning, have) = rest.split_once('|').ok_or(eyre!("no |"))?;
        let parse_numbers = |s: &str| {
            s.trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().map_err(Into::into))
                .collect::<Result<Vec<_>>>()
        };
        let winning_numbers = parse_numbers(winning)?;
        let have_numbers = parse_numbers(have)?;

        Ok(Card {
            number,
            winning_numbers,
            have_numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4() {
        assert_eq!(solve(), (20829, 12648035))
    }

    #[test]
    fn example() {
        tracing_subscriber::fmt::try_init().ok();
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
        .trim();
        let cards = parse(input);
        let part1 = part1(&cards);
        assert_eq!(part1, 13);
        assert_eq!(part2(&cards), 30);
    }
}
