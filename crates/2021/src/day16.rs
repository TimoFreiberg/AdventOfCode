#![allow(warnings)]
use core::num;
use std::collections::VecDeque;

use eyre::Result;

use crate::input;

pub fn solve() -> Result<(u64, i64)> {
    let input = input(16);
    let packet = parse(&input.trim());

    Ok((packet.sum_versions(), packet.evaluate()))
}

fn part1(input: &str) -> u64 {
    let packet = parse(input.trim());
    packet.sum_versions()
}

fn parse(input: &str) -> Packet {
    let mut bits = decode_ez(input);
    decode_packet(&mut bits)
}

fn decode_ez(input: &str) -> Bits {
    let bits = input
        .as_bytes()
        .iter()
        .flat_map(|b| match b {
            b'0' => [0, 0, 0, 0],
            b'1' => [0, 0, 0, 1],
            b'2' => [0, 0, 1, 0],
            b'3' => [0, 0, 1, 1],
            b'4' => [0, 1, 0, 0],
            b'5' => [0, 1, 0, 1],
            b'6' => [0, 1, 1, 0],
            b'7' => [0, 1, 1, 1],
            b'8' => [1, 0, 0, 0],
            b'9' => [1, 0, 0, 1],
            b'A' => [1, 0, 1, 0],
            b'B' => [1, 0, 1, 1],
            b'C' => [1, 1, 0, 0],
            b'D' => [1, 1, 0, 1],
            b'E' => [1, 1, 1, 0],
            b'F' => [1, 1, 1, 1],
            _ => panic!("Invalid hex {}", b),
        })
        .map(|b| if b == 0 { Bit::Zero } else { Bit::One })
        .collect();
    Bits(bits)
}

fn decode_packet(bits: &mut Bits) -> Packet {
    let version = bits.take(3);
    let version = bits_to_byte(&version);

    let type_id = bits.take(3);
    let type_id = bits_to_byte(&type_id);

    let contents = match type_id {
        4 => decode_literal(bits),
        _ => decode_operator(bits),
    };
    Packet {
        version,
        type_id,
        contents,
    }
}

fn decode_operator(bits: &mut Bits) -> PacketContents {
    let length_type_id = bits.pop();
    let packets = if length_type_id.is_one() {
        let number_of_subpackets = bits_to_i64(&bits.take(11));
        decode_n_packets(bits, number_of_subpackets)
    } else {
        let subpacket_length_bits = bits_to_i64(&bits.take(15)) as usize;
        decode_packets(bits.take(subpacket_length_bits))
    };

    PacketContents::Operator { packets }
}

fn decode_n_packets(bits: &mut Bits, number_of_subpackets: i64) -> Vec<Packet> {
    (0..number_of_subpackets)
        .map(|_| decode_packet(bits))
        .collect()
}

fn decode_packets(mut bits: Bits) -> Vec<Packet> {
    let mut result = Vec::new();
    while !bits.0.is_empty() {
        result.push(decode_packet(&mut bits));
    }
    result
}

fn decode_literal(bits: &mut Bits) -> PacketContents {
    let mut number = VecDeque::new();
    loop {
        let mut group = bits.take(5);
        let start = group.pop();
        number.extend(group.0);
        if !start.is_one() {
            break;
        }
    }
    PacketContents::Literal {
        number: bits_to_i64(&Bits(number)),
    }
}

fn bits_to_i64(bits: &Bits) -> i64 {
    let mut result = 0;
    let mut multiplier = 1;
    for i in bits.0.iter().rev() {
        result += i.num() as i64 * multiplier;
        multiplier *= 2;
    }
    result
}

fn bits_to_byte(bits: &Bits) -> u8 {
    assert!(
        bits.0.len() < 8,
        "{} bits don't fit into a u8",
        bits.0.len()
    );
    let mut result = 0;
    let mut multiplier = 1;
    for i in (0..bits.0.len()).rev() {
        result += bits.0[i].num() * multiplier;
        multiplier *= 2;
    }
    result
}

#[derive(Debug, PartialEq, Eq)]
struct Bits(VecDeque<Bit>);
impl Bits {
    fn pop(&mut self) -> Bit {
        self.0.pop_front().unwrap()
    }
    fn take(&mut self, n: usize) -> Bits {
        Bits(self.0.drain(0..n).collect())
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    contents: PacketContents,
}

impl Packet {
    fn sum_versions(&self) -> u64 {
        self.version as u64
            + match &self.contents {
                PacketContents::Literal { .. } => 0,
                PacketContents::Operator { packets } => {
                    packets.iter().map(|p| p.sum_versions()).sum()
                }
            }
    }
    fn evaluate(&self) -> i64 {
        match &self.contents {
            PacketContents::Literal { number } => *number,
            PacketContents::Operator { packets } => {
                let mut evaluated = packets.iter().map(Packet::evaluate);
                match self.type_id {
                    0 => evaluated.sum(),
                    1 => evaluated.product(),
                    2 => evaluated.min().unwrap(),
                    3 => evaluated.max().unwrap(),
                    5 => {
                        let first = evaluated.next().unwrap();
                        let second = evaluated.next().unwrap();
                        if first > second {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        let first = evaluated.next().unwrap();
                        let second = evaluated.next().unwrap();
                        if first < second {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        let first = evaluated.next().unwrap();
                        let second = evaluated.next().unwrap();
                        if first == second {
                            1
                        } else {
                            0
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
        }
    }
    fn visit<F>(&self, mut f: F)
    where
        F: FnMut(&Self),
    {
        f(self);
        match &self.contents {
            PacketContents::Literal { .. } => {}
            PacketContents::Operator { packets } => {
                for packet in packets {
                    packet.visit(&mut f)
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum PacketContents {
    Literal { number: i64 },
    Operator { packets: Vec<Packet> },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Bit {
    One,
    Zero,
}

impl std::fmt::Debug for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Bit::One => '1',
                Bit::Zero => '0',
            }
        )
    }
}

impl Bit {
    fn is_one(&self) -> bool {
        matches!(self, Bit::One)
    }
    fn num(&self) -> u8 {
        if self.is_one() {
            1
        } else {
            0
        }
    }
}

fn decode(input: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(input.len() / 2);
    for chunk in input.as_bytes().chunks(2) {
        match chunk {
            [a, b] => {
                let b = (decode_char(*a) << 4) + decode_char(*b);
                result.push(b);
            }
            _ => unreachable!(),
        }
    }
    result
}

fn decode_char(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => (c - b'a') + 10,
        b'A'..=b'F' => (c - b'A') + 10,
        _ => panic!("Invalid hex {}", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16() {
        let (part1, part2) = solve().unwrap();
        assert_eq!(part1, 904);
        assert_eq!(part2, 200476472872);
    }

    #[test]
    fn test_bits() {
        use Bit::*;

        let mut bits = decode_ez("69");
        dbg!(&bits);
        assert_eq!(bits.take(4), decode_ez("6"));
        assert_eq!(bits, decode_ez("9"));
        assert_eq!(bits.pop(), One);
        assert_eq!(bits, Bits(VecDeque::from_iter([Zero, Zero, One])));
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            parse("D2FE28"),
            Packet {
                version: 6,
                type_id: 4,
                contents: PacketContents::Literal { number: 2021 }
            }
        );

        assert_eq!(
            parse("38006F45291200"),
            Packet {
                version: 1,
                type_id: 6,
                contents: PacketContents::Operator {
                    packets: vec![
                        Packet {
                            version: 6,
                            type_id: 4,
                            contents: PacketContents::Literal { number: 10 }
                        },
                        Packet {
                            version: 2,
                            type_id: 4,
                            contents: PacketContents::Literal { number: 20 }
                        }
                    ]
                }
            }
        )
    }

    // #[test]
    // fn type_recursion() {
    //     let mut x = 0;
    //     parse("38006F45291200").visit(|p| x += p.version);
    // }
}
