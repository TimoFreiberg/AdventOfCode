use eyre::{eyre, Result};
use tracing::debug;

use crate::input;

pub fn solve() -> Result<(u32, u32)> {
    let input = input(4);
    let game = parse(&input)?;
    let winners = game.winners();
    Ok((part1(&winners), part2(&winners)))
}

fn part1(winners: &[Vec<(Board, u32)>]) -> u32 {
    let first = winners.first().unwrap();
    assert_eq!(first.len(), 1);
    first[0].1
}

fn part2(winners: &[Vec<(Board, u32)>]) -> u32 {
    let last = winners.last().unwrap();
    assert_eq!(last.len(), 1);
    last[0].1
}

fn parse(input: &str) -> Result<Game> {
    let mut paragraphs = input.split("\n\n");

    let numbers = paragraphs
        .next()
        .ok_or_else(|| eyre!("Missing first numbers paragraph"))?
        .split(',')
        .map(|n| Ok(n.parse::<u8>()?))
        .collect::<Result<Vec<_>>>()?;

    let boards = paragraphs
        .map(|p| {
            let rows = p
                .lines()
                .map(|l| {
                    let row = l
                        .split_whitespace()
                        .map(|tok| Ok(tok.parse::<u8>()?))
                        .collect::<Result<Vec<_>>>()?;
                    Ok(Row(row))
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(Board::new(rows))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Game { numbers, boards })
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl Game {
    fn winners(mut self) -> Vec<Vec<(Board, u32)>> {
        let mut numbers = Vec::with_capacity(self.numbers.len());
        let mut winners = Vec::with_capacity(numbers.len());
        for number in &self.numbers {
            numbers.push(*number);
            debug!("Trying {:?}", numbers);

            let (new_winners, still_losers): (Vec<_>, _) =
                self.boards.into_iter().partition(|b| b.won(&numbers));
            self.boards = still_losers;

            let new_winners: Vec<_> = new_winners
                .into_iter()
                .map(|b| {
                    let score = b.score(&numbers) * (*number as u32);
                    (b, score)
                })
                .collect();
            if !new_winners.is_empty() {
                debug!("Found winners: {:?}", winners);
                winners.push(new_winners);
            }
        }
        winners
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Board {
    rows: Vec<Row>,
}

impl Board {
    fn new(rows: Vec<Row>) -> Self {
        Self { rows }
    }
    fn won(&self, numbers: &[u8]) -> bool {
        let mut row_len = 0;
        for row in &self.rows {
            row_len = row.0.len();
            if row.0.iter().all(|n| numbers.contains(n)) {
                return true;
            }
        }

        for col_ix in 0..row_len {
            if self.rows.iter().all(|row| numbers.contains(&row.0[col_ix])) {
                return true;
            }
        }
        false
    }
    fn score(&self, numbers: &[u8]) -> u32 {
        self.rows
            .iter()
            .flat_map(|row| row.0.iter().filter(|n| !numbers.contains(*n)))
            .map(|n| *n as u32)
            .sum()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Row(Vec<u8>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_ex() {
        let game = parse(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
        )
        .unwrap();

        let winners = game.winners();

        assert_eq!(part1(&winners), 4512);
        assert_eq!(part2(&winners), 1924);
    }

    #[test]
    fn day4() {
        let (pt1, _pt2) = solve().unwrap();

        assert_ne!(pt1, 1026);
    }
}
