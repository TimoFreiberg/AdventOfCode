use std::{collections::BTreeSet, str::FromStr};

use eyre::{bail, eyre, Result};

const INPUT: &str = include_str!("../input8.txt");

pub(crate) fn solve() -> Result<()> {
    let program = parse(INPUT)?;

    println!("day8.1: {}", part1(&program));
    println!("day8.2: {}", part2(program));
    Ok(())
}

fn part1(program: &[OpCode]) -> i32 {
    match run(program, false) {
        ProgramResult::Loop(i) => i,
        ProgramResult::End(_) => panic!("Program should loop"),
    }
}

fn part2(mut program: Vec<OpCode>) -> i32 {
    let original = program.clone();

    for switch_pos in 0..program.len() {
        if let Some(switched) = program[switch_pos].switch() {
            program[switch_pos] = switched;
        } else {
            continue;
        }
        if let ProgramResult::End(i) = run(&program, false) {
            // run(&program, true);
            // eprintln!(
            //     "Terminated with switch at {} ({:?})",
            //     switch_pos, program[switch_pos]
            // );
            return i;
        }
        program = original.clone();
    }
    panic!("No solution found");
}

fn run(program: &[OpCode], debug: bool) -> ProgramResult {
    let mut accumulator = 0;
    let mut evaluated_codes = BTreeSet::new();

    let mut ix = 0i32;

    loop {
        if evaluated_codes.contains(&ix) {
            if debug {
                eprintln!("Loop at {}", ix);
            }
            return ProgramResult::Loop(accumulator);
        } else if ix as usize >= program.len() {
            if debug {
                eprintln!("Finished at {} (/{})", ix, program.len());
            }
            return ProgramResult::End(accumulator);
        }

        let op_code = program[ix as usize];

        let prev_accum = accumulator;
        evaluated_codes.insert(ix);
        match op_code {
            OpCode::Acc(i) => {
                accumulator += i;
                ix += 1;
            }
            OpCode::Jmp(i) => ix += i,
            OpCode::Nop(_) => ix += 1,
        };
        if debug {
            eprintln!(
                "({}): {:?} ({} -> {})",
                ix, op_code, prev_accum, accumulator
            );
        }
        if ix < 0 {
            panic!("Illegal position {}", ix);
        }
    }
}

enum ProgramResult {
    Loop(i32),
    End(i32),
}

fn parse(input: &str) -> Result<Vec<OpCode>> {
    input.lines().map(OpCode::from_str).collect()
}

#[derive(Clone, Copy, Debug)]
enum OpCode {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl OpCode {
    fn switch(&self) -> Option<Self> {
        match self {
            OpCode::Acc(_) => None,
            OpCode::Jmp(i) => Some(OpCode::Nop(*i)),
            OpCode::Nop(i) => Some(OpCode::Jmp(*i)),
        }
    }
}

impl FromStr for OpCode {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split_whitespace();

        let code = tokens
            .next()
            .ok_or_else(|| eyre!("Missing opcode in {:?}", s))?;
        let arg = tokens
            .next()
            .ok_or_else(|| eyre!("Missing argument in {:?}", s))?
            .parse()?;
        Ok(match code {
            "nop" => OpCode::Nop(arg),
            "acc" => OpCode::Acc(arg),
            "jmp" => OpCode::Jmp(arg),
            _ => bail!("Invalid instruction {:?}", s),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8() {
        let program = parse(INPUT).unwrap();

        assert_eq!(part1(&program), 1654);
        assert_eq!(part2(program), 833);
    }

    #[test]
    fn day8_ex() {
        let program = parse(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
jmp +1",
        )
        .unwrap();

        assert_eq!(part2(program), 8);
    }
}
