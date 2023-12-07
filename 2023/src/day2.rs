use std::str::FromStr;

use eyre::{bail, eyre, Result};

use crate::input;

pub fn solve() -> (u64, u64) {
    let input = input(2);
    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()
        .unwrap();
    let part1 = part1(
        &games,
        &Colors {
            red: 12,
            green: 13,
            blue: 14,
        },
    );
    (part1, part2(&games))
}

fn part1(games: &[Game], constraint: &Colors) -> u64 {
    games
        .iter()
        .filter(|game| game.is_possible(constraint))
        .map(|game| u64::from(game.id))
        .sum()
}

fn part2(games: &[Game]) -> u64 {
    games.iter().map(|game| game.power()).sum()
}

struct Game {
    id: u32,
    draws: Vec<Colors>,
}
impl Game {
    fn is_possible(&self, constraint: &Colors) -> bool {
        self.draws.iter().all(|draw| draw.is_subset(&constraint))
    }

    fn power(&self) -> u64 {
        let mut red_min = 0;
        let mut blue_min = 0;
        let mut green_min = 0;
        for draw in &self.draws {
            red_min = red_min.max(draw.red);
            blue_min = blue_min.max(draw.blue);
            green_min = green_min.max(draw.green);
        }
        let result = u64::from(red_min) * u64::from(blue_min) * u64::from(green_min);
        if result == 0 {
            tracing::warn!(draws = ?self.draws, "power 0");
        }
        result
    }
}

impl FromStr for Game {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = s
            .strip_prefix("Game ")
            .ok_or(eyre!("missing Game prefix"))?;
        let (id, rest) = prefix.split_once(':').ok_or(eyre!("no colon"))?;
        let id: u32 = id.parse()?;
        let draws = rest
            .split(';')
            .map(|s| s.parse())
            .collect::<eyre::Result<Vec<_>>>()?;

        Ok(Game { id, draws })
    }
}

#[derive(Default, Debug)]
struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}
impl Colors {
    fn is_subset(&self, superset: &&Colors) -> bool {
        self.red <= superset.red && self.green <= superset.green && self.blue <= superset.blue
    }
}

impl FromStr for Colors {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut draws = Colors::default();
        for draw in s.trim().split(',') {
            let next_draw: Vec<_> = draw.trim().split_whitespace().collect();
            assert_eq!(next_draw.len(), 2);
            let num: u32 = next_draw[0].parse()?;
            match next_draw[1] {
                "red" => draws.red = num,
                "green" => draws.green = num,
                "blue" => draws.blue = num,
                other => bail!("invalid color {other:?}"),
            };
        }
        Ok(draws)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2() {
        assert_eq!(solve(), (2683, 49710))
    }
}
