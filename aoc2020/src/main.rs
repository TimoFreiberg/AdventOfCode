#![feature(pattern)]
#![feature(try_blocks)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    tracing_subscriber::fmt::init();

    day1::solve().unwrap();
    day2::solve().unwrap();
    day3::solve().unwrap();
    day4::solve().unwrap();
    day5::solve().unwrap();
    day6::solve().unwrap();
    day7::solve().unwrap();
    day8::solve().unwrap();
}

#[cfg(test)]
pub mod tests {

    pub fn init_logger() {
        let _ = tracing_subscriber::fmt::try_init();
    }
}
