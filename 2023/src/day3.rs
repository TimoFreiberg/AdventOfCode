use crate::input;

pub fn solve() -> (u64, u64) {
    let input = input(3);
    let schematic = parse(&input);
    (
        schematic.numbers_next_to_symbols().sum(),
        schematic.gear_ratios().sum(),
    )
}

fn parse(input: &str) -> Schematic {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    'line: for (x, line) in input.lines().enumerate() {
        let mut indexed = line.chars().enumerate();
        loop {
            match indexed.next() {
                Some((y, c)) if c.is_digit(10) => {
                    let mut coords = vec![Coord { x, y }];
                    let mut number = c.to_string();
                    for (y, c) in indexed.clone().take_while(|(_, c)| c.is_digit(10)) {
                        // advance the main line iterator
                        indexed.next();

                        coords.push(Coord { x, y });
                        number.push(c);
                    }
                    numbers.push(Number {
                        val: number.parse().unwrap(),
                        coords,
                    });
                }
                Some((_, '.')) => {}
                Some((y, _)) => symbols.push(Coord { x, y }),
                None => continue 'line,
            }
        }
    }
    Schematic { numbers, symbols }
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Coord>,
}

impl Schematic {
    fn numbers_next_to_symbols(&self) -> impl Iterator<Item = u64> + '_ {
        self.numbers
            .iter()
            .filter(|num| {
                let mut matching_symbol = None;
                let result = self.symbols.iter().any(|sym| {
                    sym.neighbors().iter().any(|coord| {
                        let matches = num.coords.contains(coord);
                        if matches {
                            matching_symbol = Some(sym);
                        }
                        matches
                    })
                });
                if result {
                    tracing::debug!(?matching_symbol, ?num, "number is next to symbol");
                } else {
                    tracing::debug!(?num, "number is not next to any symbol");
                }
                result
            })
            .map(|num| num.val)
    }
    fn gear_ratios(&self) -> impl Iterator<Item = u64> + '_ {
        self.symbols.iter().filter_map(|sym| {
            // let mut matching_symbol = None;
            let neighbors = self
                .numbers
                .iter()
                .filter(|num| {
                    sym.neighbors().iter().any(|coord| {
                        let matches = num.coords.contains(coord);
                        if matches {
                            // matching_symbol = Some(sym);
                        }
                        matches
                    })
                })
                .collect::<Vec<_>>();
            if neighbors.len() == 2 {
                Some(neighbors.into_iter().map(|num| num.val).product())
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone)]
struct Number {
    val: u64,
    coords: Vec<Coord>,
}

#[derive(PartialEq, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbors(&self) -> Vec<Coord> {
        let Coord { x, y } = self;
        let mut neighbors = Vec::new();
        for x in [x - 1, *x, x + 1] {
            for y in [y - 1, *y, y + 1] {
                if x == self.x && y == self.y {
                    continue;
                }
                neighbors.push(Coord { x, y });
            }
        }
        neighbors
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3() {
        assert_eq!(solve(), (554003, 87263515))
    }

    #[test]
    fn example_pt1() {
        tracing_subscriber::fmt::try_init().ok();
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
        .trim();
        let schematic = parse(input);
        tracing::info!(?schematic);

        assert_eq!(schematic.numbers_next_to_symbols().sum::<u64>(), 4361);
    }
}
