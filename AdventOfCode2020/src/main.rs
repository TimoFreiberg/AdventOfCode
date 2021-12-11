#![feature(pattern)]
#![feature(try_blocks)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    day1::solve().unwrap();
    day2::solve().unwrap();
    day3::solve().unwrap();
    day4::solve().unwrap();
    day5::solve().unwrap();
    day6::solve().unwrap();
    day7::solve().unwrap();
}

#[cfg(test)]
pub mod tests {
    use env_logger::Env;

    pub fn init_logger() {
        let _ = env_logger::try_init_from_env(Env::default().default_filter_or("debug"));
    }
}
