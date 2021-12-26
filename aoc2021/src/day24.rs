use std::{collections::BTreeMap, convert::Infallible, iter, str::FromStr};

use eyre::{bail, Result};
use itertools::Itertools;
use tracing::debug;

use crate::input;

pub fn solve() -> Result<(i64, i64)> {
    let input = input(24);
    let program = parse(INPUT);

    Ok((part1(program), 0))
}

const INPUT: &str = "\
inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// these three are replaced by 
// add x 1
// X is 1 here!
// as w is always <=9
// add x 11
// eql x w
// eql x 0
// mul y 0
// add y 25
// x is one
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 5
// mul y x
// add z y
add z w
add z 5
// -> finally, (x: 1, z: (input1+5))
// (the x part is optimized out as x would be reset to 0 immediately anyway)

inp w
// mul x 0
// x: (input1+5)
// add x z
// x can be at most 14
// mod x 26
// div z 1
// add x 13
// (input1+5+13) is always gonna be larger than 0..=9, so x will be set to 1 after this
// eql x w
// eql x 0
// new:
add x 1
// mul y 0
add y 25
// x is one
// mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -1
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -7
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -2
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -2
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
";

fn part1(program: Program) -> i64 {
    for input in numbers_without_zeroes() {
        let program = program.clone();
        if let Ok(result) = program.run(input.iter().copied()) {
            if result == 0 {
                return input.iter().join("").parse().unwrap();
            }
        }
    }
    panic!("No valid program")
}

fn parse(input: &str) -> Program {
    let code = input
        .lines()
        .filter_map(|s| {
            if s.is_empty() || s.starts_with("//") {
                None
            } else {
                Some(s.parse().unwrap())
            }
        })
        .collect();
    Program::new(code)
}

#[derive(Clone)]
struct Program {
    code: Vec<Instruction>,
    alu: Alu,
}

impl Program {
    fn new(code: Vec<Instruction>) -> Self {
        Self {
            code,
            alu: Default::default(),
        }
    }
    fn run(mut self, input: impl IntoIterator<Item = i64>) -> Result<i64> {
        let mut input = input.into_iter();
        for instruction in self.code {
            self.alu.process(instruction, &mut input)?;
        }
        debug!("{:?}", self.alu);
        Ok(*self.alu.values.get(&Var::Z).unwrap())
    }
}

#[derive(Default, Debug, Clone)]
struct Alu {
    values: BTreeMap<Var, i64>,
}

impl Alu {
    fn process(
        &mut self,
        instruction: Instruction,
        mut input: impl Iterator<Item = i64>,
    ) -> Result<()> {
        match instruction {
            Instruction::Inp(var) => {
                self.values.insert(var, input.next().unwrap());
            }
            Instruction::Add(var, arg) => {
                let result = self.read(var) + self.read_arg(arg);
                self.values.insert(var, result);
            }
            Instruction::Mul(var, arg) => {
                let result = self.read(var) * self.read_arg(arg);
                self.values.insert(var, result);
            }
            Instruction::Div(var, arg) => {
                let arg2 = self.read_arg(arg);
                if arg2 == 0 {
                    bail!("Div by 0");
                }
                let result = self.read(var) / arg2;
                self.values.insert(var, result);
            }
            Instruction::Mod(var, arg) => {
                let arg1 = self.read(var);
                let arg2 = self.read_arg(arg);
                if arg1 < 0 || arg2 <= 0 {
                    bail!("Invalid mod {} % {}", arg1, arg2);
                }
                let result = arg1 % arg2;
                self.values.insert(var, result);
            }
            Instruction::Eql(var, arg) => {
                let result = if self.read(var) == self.read_arg(arg) {
                    1
                } else {
                    0
                };
                self.values.insert(var, result);
            }
        }
        Ok(())
    }

    fn read(&self, var: Var) -> i64 {
        self.values.get(&var).copied().unwrap_or(0)
    }
    fn read_arg(&self, arg: Arg) -> i64 {
        match arg {
            Arg::Lit(i) => i,
            Arg::Var(var) => self.read(var),
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Inp(Var),
    Add(Var, Arg),
    Mul(Var, Arg),
    Div(Var, Arg),
    Mod(Var, Arg),
    Eql(Var, Arg),
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        use Var::*;
        let mut tokens = s.split_ascii_whitespace();
        let op = tokens.next().unwrap();
        let var = |t: Option<&str>| match t.unwrap().chars().next().unwrap() {
            'w' => W,
            'x' => X,
            'y' => Y,
            'z' => Z,
            _ => {
                panic!("Invalid var {:?}", t)
            }
        };
        let arg = |t: Option<&str>| match t.unwrap().parse() {
            Ok(num) => Arg::Lit(num),
            _ => Arg::Var(var(t)),
        };
        let var = var(tokens.next());
        Ok(match op {
            "inp" => Inp(var),
            "add" => Add(var, arg(tokens.next())),
            "mul" => Mul(var, arg(tokens.next())),
            "div" => Div(var, arg(tokens.next())),
            "mod" => Mod(var, arg(tokens.next())),
            "eql" => Eql(var, arg(tokens.next())),
            _ => panic!("invalid op {:?}", op),
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Var {
    W,
    X,
    Y,
    Z,
}

#[derive(Clone, Copy)]
enum Arg {
    Lit(i64),
    Var(Var),
}

fn numbers_without_zeroes() -> impl Iterator<Item = Vec<i64>> {
    iter::repeat((1..=9).rev())
        .take(14)
        .multi_cartesian_product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let program = parse(
            "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2",
        );
        assert_eq!(program.run(vec![7]).unwrap(), 1);
    }

    #[test]
    fn numbers_seq() {
        let mut numbers = numbers_without_zeroes();
        assert_eq!(
            numbers.next().unwrap(),
            vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            numbers.next().unwrap(),
            vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 8]
        );
    }
}
