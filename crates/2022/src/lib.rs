#![feature(never_type)]
#![feature(iter_array_chunks)]

use std::{env, fs};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn input(day: u32) -> String {
    fs::read_to_string(format!("inputs/input{day}.csv")).unwrap()
}

pub fn init() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "warn");
    }
    tracing_subscriber::fmt::try_init().ok();
}
