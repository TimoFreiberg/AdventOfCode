use std::{collections::VecDeque, iter, str::FromStr};

use tracing::debug;

use crate::input;

pub fn solve() -> (i64, String) {
    let input = input(10);
    (part1(&input), part2(&input))
}

fn part1(input: &str) -> i64 {
    let cpu = Cpu::default();
    let ops = parse(input);
    cpu.run(ops)
        .filter(|cpu| (cpu.tick as i32 - 20) % 40 == 0)
        .map(|cpu| {
            let result = cpu.tick as i64 * cpu.register_x as i64;
            debug!(?result, ?cpu);
            result
        })
        .sum()
}

fn part2(input: &str) -> String {
    let cpu = Cpu::default();
    let ops = parse(input);
    let mut states = cpu.run(ops);
    let mut result = Vec::new();
    for _ in 0..6 {
        let mut line = String::new();
        for col in 0..40 {
            let state = states.next().unwrap();
            match state.register_x - col {
                -1 | 0 | 1 => {
                    line.push('#');
                }
                _ => {
                    line.push('.');
                }
            }
        }
        result.push(line)
    }
    result.join("\n")
}

fn parse(input: &str) -> VecDeque<Op> {
    input
        .lines()
        .map(Op::from_str)
        .collect::<Result<VecDeque<_>, _>>()
        .unwrap()
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Noop,
    AddX(i32),
}
impl FromStr for Op {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with("noop") {
            Op::Noop
        } else if s.starts_with("addx") {
            let (_, amount) = s.split_once(' ').unwrap();
            Op::AddX(amount.parse().unwrap())
        } else {
            panic!("Illegal op {s:?}")
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Cpu {
    register_x: i32,
    op: Option<(usize, Op)>,
    tick: u32,
}

impl Cpu {
    fn run(self, mut ops: VecDeque<Op>) -> impl Iterator<Item = Cpu> {
        let mut cpu = self;
        iter::from_fn(move || {
            // start cycle
            if cpu.op.is_none() {
                cpu.op = ops.pop_front().map(|o| (0, o));
                // debug!(next_op = ?cpu.op);
            }
            if cpu.op.is_none() {
                None
            } else {
                let result = cpu;
                cpu.tick();
                Some(result)
            }
        })
    }
    fn tick(&mut self) {
        self.tick += 1;
        match &mut self.op {
            Some((_, Op::Noop)) => self.op = None,
            Some((count, Op::AddX(i))) => {
                *count += 1;
                if *count >= 2 {
                    // goes into effect at the end
                    self.register_x += *i;
                    self.op = None;
                }
            }
            None => panic!("No op in tick"),
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            register_x: 1,
            op: None,
            tick: 1,
        }
    }
}

#[test]
fn example() {
    crate::init();
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    assert_eq!(part1(input), 13140);
    let pt2 = part2(input);
    println!("{pt2}");
    assert_eq!(
        pt2,
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
}

#[test]
fn day10() {
    assert_eq!(
        solve(),
        (
            11960,
            "####...##..##..####.###...##..#....#..#.
#.......#.#..#.#....#..#.#..#.#....#..#.
###.....#.#....###..#..#.#....#....####.
#.......#.#....#....###..#.##.#....#..#.
#....#..#.#..#.#....#....#..#.#....#..#.
####..##...##..#....#.....###.####.#..#."
                .to_string()
        )
    )
}
