fn main() {
    aoc22::init();
    println!("day1: {:?}", aoc22::day1::solve());
    println!("day2: {:?}", aoc22::day2::solve());
    println!("day3: {:?}", aoc22::day3::solve());
    println!("day4: {:?}", aoc22::day4::solve());
    println!("day5: {:?}", aoc22::day5::solve());
    println!("day6: {:?}", aoc22::day6::solve());
    println!("day7: {:?}", aoc22::day7::solve());
    println!("day8: {:?}", aoc22::day8::solve());
    println!("day9: {:?}", aoc22::day9::solve());
    let (day10_1, day10_2) = aoc22::day10::solve();
    println!("day10.1: {day10_1:?}, day10.2:");
    println!("{day10_2}");
    println!("day11: {:?}", aoc22::day11::solve());
    println!("day12: {:?}", aoc22::day12::solve());
}
