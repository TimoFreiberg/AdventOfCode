use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub fn main() {
    tracing_subscriber::fmt::init();
    println!("day1: {:?}", day1::solve());
    println!("day2: {:?}", day2::solve());
    // slow
    // println!("day3: {:?}", day3::solve());
    println!("day4: {:?}", day4::solve());
    // slow
    // println!("day5: {:?}", day5::solve());
}

pub fn input(day: u32) -> String {
    fs::read_to_string(format!("inputs/day{day}.txt")).expect("no input file")
}
