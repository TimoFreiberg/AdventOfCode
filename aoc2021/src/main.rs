use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    tracing_subscriber::fmt::init();
    println!("day1: {:?}", day1::solve().unwrap());
    println!("day2: {:?}", day2::solve().unwrap());
    println!("day3: {:?}", day3::solve().unwrap());
    println!("day4: {:?}", day4::solve().unwrap());
    println!("day5: {:?}", day5::solve().unwrap());
    println!("day6: {:?}", day6::solve().unwrap());
    println!("day7: {:?}", day7::solve().unwrap());
    println!("day8: {:?}", day8::solve().unwrap());
    println!("day9: {:?}", day9::solve().unwrap());
}

fn input(day: usize) -> String {
    let path = format!("input{}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
}
