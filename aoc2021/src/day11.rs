use std::collections::BTreeSet;

use eyre::Result;
use tracing::debug;

pub fn solve() -> Result<(usize, u64)> {
    let input = "4871252763
8533428173
7182186813
2128441541
3722272272
8751683443
3135571153
5816321572
2651347271
7788154252
";
    let mut octopuses = parse(input);

    Ok(parts(&mut octopuses))
}

fn parts(octopuses: &mut Octopuses) -> (usize, u64) {
    let octopus_count = octopuses.0.iter().map(|l| l.len()).sum();
    let mut part1 = 0;
    let mut part2 = None;
    for i in 1.. {
        if i > 100 && part2.is_some() {
            break;
        }
        let flashes = octopuses.step().len();
        if i <= 100 {
            part1 += flashes;
        }
        if flashes == octopus_count {
            part2 = Some(i);
        }
    }
    (part1, part2.unwrap())
}

fn parse(input: &str) -> Octopuses {
    let octopuses = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect();
    Octopuses(octopuses)
}

#[derive(PartialEq, Eq)]
struct Octopuses(Vec<Vec<u8>>);

impl std::fmt::Debug for Octopuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Octopuses:")?;
        for line in &self.0 {
            for c in line {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Octopuses {
    fn step(&mut self) -> BTreeSet<(usize, usize)> {
        for row in &mut self.0 {
            for octopus in row {
                *octopus += 1;
            }
        }

        let mut flashed = BTreeSet::new();
        for x in 0..self.0.len() {
            for y in 0..self.0[x].len() {
                if self.at(x, y).copied().unwrap() > 9 {
                    self.flash(x, y, &mut flashed);
                }
            }
        }

        for (x, y) in &flashed {
            if let Some(flashed_octopus) = self.at(*x, *y) {
                assert!(
                    *flashed_octopus > 9,
                    "Flashed octopus with value {}",
                    flashed_octopus
                );
                *flashed_octopus = 0;
            } else {
                panic!("Invalid flashed octopus at {},{}", x, y);
            }
        }

        flashed
    }

    fn flash(&mut self, x: usize, y: usize, flashed: &mut BTreeSet<(usize, usize)>) {
        if !flashed.insert((x, y)) {
            return;
        }
        debug!("Flashing {},{}", x, y);
        for (x1, y1) in neighbors(x, y) {
            if let Some(neighbor) = self.at(x1, y1) {
                debug!("Flashed neighbor of {},{}:  {},{}", x, y, x1, y1);
                *neighbor += 1;
                if *neighbor > 9 {
                    self.flash(x1, y1, flashed);
                }
            }
        }
    }

    fn at(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        self.0.get_mut(x)?.get_mut(y)
    }
}

fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let start_x = if x == 0 { x } else { x - 1 };
    let start_y = if y == 0 { y } else { y - 1 };
    (start_x..=x + 1)
        .flat_map(|x1| (start_y..=y + 1).map(move |y1| (x1, y1)))
        .filter(|(x1, y1)| *x1 != x || *y1 != y)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11() {
        let (part1, part2) = solve().unwrap();

        assert_eq!(part1, 1747);
        assert_eq!(part2, 505);
    }

    #[test]
    fn day11_ex() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut octopuses = parse(input);
        assert_eq!(octopuses.step().len(), 0);

        assert_eq!(
            octopuses,
            parse(
                "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637"
            )
        );

        eprintln!("{:?}", octopuses);

        assert_ne!(octopuses.step().len(), 0);

        assert_eq!(
            octopuses,
            parse(
                "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848"
            )
        );
    }

    #[test]
    fn day11_smol_ex() {
        let mut octopuses = parse(
            "11111
19991
19191
19991
11111",
        );
        assert_eq!(octopuses.step().len(), 9);

        assert_eq!(
            octopuses,
            parse(
                "34543
40004
50005
40004
34543"
            )
        );
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(
            neighbors(5, 10),
            vec![
                (4, 9),
                (4, 10),
                (4, 11),
                (5, 9),
                (5, 11),
                (6, 9),
                (6, 10),
                (6, 11)
            ]
        );

        assert!(dbg!(neighbors(0, 2)).contains(&(1, 1)));
    }
}
