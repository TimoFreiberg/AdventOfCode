use std::fs;

mod day1;

fn main() {
    println!("day1: {:?}", day1::solve().unwrap());
}

fn input(day: usize) -> String {
    let path = format!("input{}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
}
