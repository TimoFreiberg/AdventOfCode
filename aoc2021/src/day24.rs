use std::iter;

use eyre::Result;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn solve() -> Result<(String, String)> {
    Ok((part1(), part2()))
}

#[allow(clippy::too_many_arguments, clippy::let_and_return)]
fn optimized_program(
    input1: i64,
    input2: i64,
    input3: i64,
    input4: i64,
    input5: i64,
    input6: i64,
    input7: i64,
    input8: i64,
    input9: i64,
    input10: i64,
    input11: i64,
    input12: i64,
    input13: i64,
    input14: i64,
) -> Option<Vec<i64>> {
    let z1 = input1 + 5;
    let z2 = (z1 * 26) + input2 + 5;
    let z3 = (z2 * 26) + input3 + 1;
    let z4 = (z3 * 26) + input4 + 15;
    let z5 = (z4 * 26) + input5 + 2;

    // input6 == input5 + 1
    let x6 = if (z5 % 26) - 1 == input6 { 0 } else { 1 };
    let y6_1 = (25 * x6) + 1;
    let y6_2 = (input6 + 2) * x6;
    let z6 = (z5 / 26) * y6_1 + y6_2;

    let z7 = (z6 * 26) + input7 + 5;

    // input8 == input7 - 3
    let x8 = if (z7 % 26) - 8 == input8 { 0 } else { 1 };
    let y8_1 = (25 * x8) + 1;
    let y8_2 = (input8 + 8) * x8;
    let z8 = (z7 / 26) * y8_1 + y8_2;

    // input9 == input4 + 8
    let x9 = if (z8 % 26) - 7 == input9 { 0 } else { 1 };
    let y9_1 = (25 * x9) + 1;
    let y9_2 = (input9 + 14) * x9;
    let z9 = (z8 / 26) * y9_1 + y9_2;

    // input10 == input3 - 7
    let x10 = if (z9 % 26) - 8 == input10 { 0 } else { 1 };
    let y10_1 = (25 * x10) + 1;
    let y10_2 = (input10 + 12) * x10;
    let z10 = (z9 / 26) * y10_1 + y10_2;

    let z11 = (z10 * 26) + input11 + 7;

    // input12 == input11 + 5
    let x12 = if (z11 % 26) - 2 == input12 { 0 } else { 1 };
    let y12_1 = (25 * x12) + 1;
    let y12_2 = (input12 + 14) * x12;
    let z12 = (z11 / 26) * y12_1 + y12_2;

    // input13 == input2 +3
    let x13 = if (z12 % 26) - 2 == input13 { 0 } else { 1 };
    let y13_1 = (25 * x13) + 1;
    let y13_2 = (input13 + 13) * x13;
    let z13 = (z12 / 26) * y13_1 + y13_2;

    // input14 == input1 - 8
    let x14 = if (z13 % 26) - 13 == input14 { 0 } else { 1 };
    let y14_1 = (25 * x14) + 1;
    let y14_2 = (input14 + 6) * x14;
    let z14 = (z13 / 26) * y14_1 + y14_2;

    if z14 == 0 {
        Some(vec![
            input1, input2, input3, input4, input5, input6, input7, input8, input9, input10,
            input11, input12, input13, input14,
        ])
    } else {
        None
    }
}

fn part1() -> String {
    first_matching_serial_number(
        iter::repeat((1..=9).rev())
            // 6 args
            .take(6)
            .multi_cartesian_product(),
    )
}

fn part2() -> String {
    first_matching_serial_number(
        iter::repeat(1..=9)
            // 6 args
            .take(6)
            .multi_cartesian_product(),
    )
}

fn first_matching_serial_number(
    serial_numbers: impl Iterator<Item = Vec<i64>> + Send + Sync,
) -> String {
    if let Some(success) = serial_numbers.par_bridge().find_map_first(|input| {
        if input[0] > 6 || input[1] < 8 || input[2] > 8 || input[3] < 4 || input[4] > 4 {
            return None;
        }
        // input6 == input5 + 1
        // input8 == input7 - 3
        // input12 == input11 + 5
        optimized_program(
            9,
            input[0],
            input[1],
            1,
            input[2],
            input[2] + 1,
            input[3],
            input[3] - 3,
            9,
            input[1] - 7,
            input[4],
            input[4] + 5,
            input[0] + 3,
            1,
        )
        // 96918996924991
        // 96918996924991
    }) {
        assert!(
            success.iter().all(|&num| num > 0 && num < 10),
            "Invalid numbers: {:?}",
            success
        );
        let s = success.iter().copied().join("");
        assert_eq!(success.len(), 14);
        return s.parse().unwrap();
    }

    panic!("No valid program")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day24() {
        let (part1, part2) = solve().unwrap();
        assert_eq!(part1, "96918996924991");
        assert_eq!(part2, "91811241911641");
    }

    #[test]
    fn numbers_seq() {
        let mut numbers = iter::repeat((1..=9).rev())
            .take(7)
            .multi_cartesian_product();
        assert_eq!(
            numbers.next().unwrap(),
            vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
        );
        assert_eq!(
            numbers.next().unwrap(),
            vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 8]
        );
    }
}

const _INPUT: &str = "\
// input1
inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// these three are replaced by 
// add x 1
// X is 1 here!
// as w is always <=9
// add x 11
// eql x w
// eql x 0
// mul y 0
// add y 25
// x is one
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 5
// mul y x
// add z y
add z w
add z 5
// -> finally, (x: 1, z: (input1+5))
// (the x part is optimized out as x would be reset to 0 immediately anyway)

// input2
inp w
// mul x 0
// x: (input1+5)
// add x z
// x can be at most 14
// mod x 26
// div z 1
// add x 13
// (input1+5+13) is always gonna be larger than 0..=9, so x will be set to 1 after this
// eql x w
// eql x 0
// new:
// add x 1
// mul y 0
// add y 25
// x is one
// mul y x
// add y 1
mul z 26
// mul y 0
// add y w
// add y 5
// mul y x
// add z y
add z w
add z 5
// z: (input1+5)*26 + input2 + 5

// input3
inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// add x 12
// eql x w
// eql x 0
// again, all writes on x are obsoleted by this as x is incremented by 12 before the comparison
// add x 1
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
mul z 26
// add y w
// add y 1
// mul y x
// add z y
add z w
add z 1
// z: ((input1+5)*26 + input2 + 5)*26 + input3

// input4
inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// add x 15
// eql x w
// eql x 0
// add x 1
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
mul z 26
// add y w
// add y 15
// mul y x
// add z y
add z w
add z 15
// z: (((input1+5)*26 + input2 + 5)*26 + input3) * 26 + input4 + 15

// input5
inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// add x 10
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
mul z 26
// mul y 0
// add y w
// add y 2
// mul y x
// add z y
add z w
add z 2
// z: ((((input1+5)*26 + input2 + 5)*26 + input3) * 26 + input4 + 15) * 26 + input5 + 2

// input6
inp w
// mul x 0
add x z
mod x 26
div z 26
add x -1
eql x w
eql x 0
// mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
mul y 0
// x: if (previous_z % 26) - 1 == input6 {0} else {1}
// y1: (25*x)+1
// y2: (input6+2)*x
// z6: (((((input1+5)*26 + input2 + 5)*26 + input3) * 26 + input4$ + 15) * 26 + input5 + 2) / 26 * y1 + y2
// TODO: make (z_after_input_5 % 26) - 1 to 0 - eh, doesn't solve it completely

// input7
inp w
mul x 0
// add x z
// mod x 26
// div z 1
// add x 14
// eql x w
// eql x 0
// add x 1
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
mul z 26
// mul y 0
// add y w
// add y 5
// mul y x
// add z y
add z w
add z 5
// z7: (z6*26)+input7+5

// input8
inp w
// mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
// mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
mul x 0
mul y 0
// x: if (z % 26) - 8 == input8 {0} else {1}
// y1: (25 * x) + 1
// y2: (input8 + 8) * x
// z8: (z7 / 26) * y1 + y2

// input9
inp w
// mul x 0
add x z
mod x 26
div z 26
add x -7
eql x w
eql x 0
// mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y
mul y 0
mul x 0
// x: if (z8 % 26) - 7 == input9 {0} else {1}
// y1: (25 * x) + 1
// y2: (input9 + 14) * x
// z9: (z8 / 26) * y1 + y2

// input10
inp w
// mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
// mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
// x: if (z9 % 26) - 8 == input10 {0} else {1}
// y1: (25 * x) + 1
// y2: (input10 + 12) * x
// z10: (z9 / 26) * y1 + y2

// input11
inp w
mul x 0
// add x z
// mod x 26
// div z 1
// add x 11
// eql x w
// eql x 0
// add x 1
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
mul z 26
// mul y 0
// add y w
// add y 7
// mul y x
// add z y
add z w
add z 7
// z11: (z10 * 26) + input11 + 7

// input12
inp w
mul x 0
add x z
mod x 26
div z 26
add x -2
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y
// x: if (z11 % 26) - 2 == input12 {0} else {1}
// y1: 25*x + 1
// y2: (input12 + 14) * x 
// z12: (z11 / 26) * y1 + y2

// input13
inp w
mul x 0
add x z
mod x 26
div z 26
add x -2
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
// x: if (z12 % 26) - 2 == input13 {0} else {1}
// y1: 25*x + 1
// y2: (input13 + 13) * x
// z13: (z12 / 26) * y1 + y2

// input14
inp w
mul x 0
add x z
mod x 26
div z 26
add x -13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
// x: if (z13 % 26) - 13 == input14 {0} else {1}
// y1: 25*x + 1
// y2: (input14 + 6) * x
// z14: (z13 / 26) * y1 + y2
";
