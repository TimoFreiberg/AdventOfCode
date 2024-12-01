use std::fs;

mod day1;

pub fn main() {
    tracing_subscriber::fmt::init();
    println!("day1: {:?}", day1::solve());
}

pub fn input(day: u32) -> String {
    fs::read_to_string(format!("inputs/2024/{day}")).expect("no input file")
}
