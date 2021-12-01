use std::fs;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day24;
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
    println!("day10: {:?}", day10::solve().unwrap());
    println!("day11: {:?}", day11::solve().unwrap());
    println!("day12: {:?}", day12::solve().unwrap());
    let (day13_1, day13_2) = day13::solve().unwrap();
    println!("day13.1: {:?}", day13_1);
    println!("day13.2:\n{}", day13_2);
    println!("day14: {:?}", day14::solve().unwrap());
    println!("day15: {:?}", day15::solve().unwrap());
    println!("day16: {:?}", day16::solve().unwrap());
    println!("day17: {:?}", day17::solve().unwrap());
    println!("day18: {:?}", day18::solve().unwrap());
    println!("day24: {:?}", day24::solve().unwrap());
}

fn input(day: usize) -> String {
    let path = format!("input{}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
}
