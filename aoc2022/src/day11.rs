use std::{cmp, str::FromStr};

use itertools::Itertools;

use crate::input;

pub fn solve() -> (u64, u64) {
    let input = input(11);
    (part1(&input), part2(&input))
}

fn part1(input: &str) -> u64 {
    let mut monkeys = MonkeyBusiness {
        monkeys: parse(&input),
        relief_factor: ReliefFactor::Div(3),
    };
    for _ in 0..20 {
        monkeys.round();
    }
    let (most_active, second_most_active) = monkeys.most_active_monkeys();
    most_active * second_most_active
}

fn part2(input: &str) -> u64 {
    let monkeys = parse(&input);
    let lcm = monkeys.iter().map(|m| m.test.divisor).product();
    let mut monkeys = MonkeyBusiness {
        monkeys,
        relief_factor: ReliefFactor::Modulo(lcm),
    };
    for _ in 0..10_000 {
        monkeys.round();
    }
    let (most_active, second_most_active) = monkeys.most_active_monkeys();
    most_active * second_most_active
}

type Item = u64;

#[derive(Clone)]
struct Monkey {
    id: MonkeyId,
    items: Vec<Item>,
    op: Operation,
    test: Test,
    monkey_business: u64,
}

#[derive(Clone)]
struct Operation {
    l: Operand,
    op: Operator,
    r: Operand,
}
impl Operation {
    fn run(&self, item: Item) -> Item {
        let l = self.l.value(item);
        let r = self.r.value(item);
        match self.op {
            Operator::Add => l.checked_add(r).unwrap(),
            Operator::Mul => l
                .checked_mul(r)
                .unwrap_or_else(|| panic!("{l} * {r} overflows")),
        }
    }
}

impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.l, self.op, self.r)
    }
}

#[derive(Clone)]
enum Operator {
    Add,
    Mul,
}

impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Mul => write!(f, "*"),
        }
    }
}

#[derive(Clone)]
struct Test {
    divisor: Item,
    if_true: MonkeyId,
    if_false: MonkeyId,
}

impl std::fmt::Debug for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "if div by {}: {} else {}",
            self.divisor, self.if_true, self.if_false
        )
    }
}
impl Test {
    fn run(&self, item: &Item) -> MonkeyId {
        if item % &self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

impl Monkey {
    fn turn(&mut self, relief_factor: ReliefFactor) -> Vec<(Item, MonkeyId)> {
        self.items
            .drain(..)
            .map(|item| {
                let inspected = self.op.run(item);
                let bored_relieved = relief_factor.run(inspected);
                self.monkey_business += 1;
                let target = self.test.run(&bored_relieved);
                // debug!(
                //     "monkey {} inspects {}, {:?} = {}, /3 = {}, {:?} -> {}",
                //     self.id, item, self.op, inspected, bored_relieved, self.test, target
                // );
                (bored_relieved, target)
            })
            .collect_vec()
    }
}

struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
    relief_factor: ReliefFactor,
}
impl MonkeyBusiness {
    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            let items = self.monkeys[i].turn(self.relief_factor);
            for (item, target_id) in items {
                let target = &mut self.monkeys[target_id as usize];
                assert_eq!(target.id, target_id);
                target.items.push(item);
            }
        }
    }
    fn most_active_monkeys(&self) -> (u64, u64) {
        let mut monkeys = self.monkeys.clone();
        monkeys.sort_unstable_by_key(|m| cmp::Reverse(m.monkey_business));

        (monkeys[0].monkey_business, monkeys[1].monkey_business)
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|block| Monkey::from_str(block).unwrap())
        .collect()
}

impl FromStr for Monkey {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id: u32 = {
            let (_monkey, id) = lines.next().unwrap().split_once(' ').unwrap();
            let id = id.strip_suffix(':').unwrap();
            id.parse().unwrap()
        };
        let items: Vec<_> = {
            let (_, items) = lines.next().unwrap().split_once(':').unwrap();
            items
                .split(',')
                .map(|i| i.trim().parse::<Item>().unwrap())
                .collect()
        };

        let op = {
            let op = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Operation: ")
                .unwrap();
            let mut expr = op.strip_prefix("new = ").unwrap().split_whitespace();
            let operand = |s: &str| match s {
                "old" => Operand::Old,
                num => {
                    let num = num.parse::<Item>().unwrap();
                    Operand::Lit(num)
                }
            };
            let l = operand(expr.next().unwrap());
            let op = match expr.next().unwrap() {
                "+" => Operator::Add,
                "*" => Operator::Mul,
                other => panic!("Invalid op {other:?}"),
            };
            let r = operand(expr.next().unwrap());
            Operation { l, op, r }
        };

        let test = {
            let divisor = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse::<Item>()
                .unwrap();
            let if_true = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("If true: throw to monkey ")
                .unwrap()
                .parse::<MonkeyId>()
                .unwrap();
            let if_false = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("If false: throw to monkey ")
                .unwrap()
                .parse::<MonkeyId>()
                .unwrap();
            Test {
                divisor,
                if_true,
                if_false,
            }
        };

        Ok(Monkey {
            id,
            items,
            op,
            test,
            monkey_business: 0,
        })
    }
}

#[derive(Clone, Copy)]
enum ReliefFactor {
    Div(Item),
    Modulo(Item),
}

impl ReliefFactor {
    fn run(&self, item: Item) -> Item {
        match self {
            ReliefFactor::Div(d) => item / d,
            ReliefFactor::Modulo(m) => item % m,
        }
    }
}

#[derive(Clone)]
enum Operand {
    Old,
    Lit(Item),
}

impl std::fmt::Debug for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Old => write!(f, "old"),
            Self::Lit(lit) => write!(f, "{}", lit),
        }
    }
}
impl Operand {
    fn value(&self, old: Item) -> Item {
        match self {
            Operand::Old => old,
            Operand::Lit(lit) => *lit,
        }
    }
}

type MonkeyId = u32;

#[test]
fn example() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    assert_eq!(part1(input), 10605);
}
