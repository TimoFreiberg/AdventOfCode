use std::fs;

mod day1;
mod day2;
mod day3;

pub fn main() {
    tracing_subscriber::fmt::init();
    println!("day1: {:?}", day1::solve());
    println!("day2: {:?}", day2::solve());
    println!("day3: {:?}", day3::solve());
}

pub fn input(day: u32) -> String {
    fs::read_to_string(format!("inputs/day{day}.csv")).expect("no input file")
}
