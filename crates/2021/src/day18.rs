use eyre::Result;
use itertools::Itertools;
use tracing::debug;

use crate::input;

pub fn solve() -> Result<(i32, i32)> {
    let input = input(18);
    let numbers = parse(&input);

    Ok((part1(numbers.clone()), part2(numbers)))
}

fn part1(numbers: Vec<Number>) -> i32 {
    let sum = sum(numbers);
    let tree = NumberTree::from(&sum);
    tree.magnitude()
}

fn part2(numbers: Vec<Number>) -> i32 {
    let mut max = 0;
    for num1 in &numbers {
        for num2 in &numbers {
            if num1 == num2 {
                continue;
            }
            let mut sum = num1.clone();
            sum.add(num2.clone());
            sum.reduce();
            max = max.max(NumberTree::from(&sum).magnitude());
        }
    }
    max
}

fn sum(numbers: Vec<Number>) -> Number {
    let mut numbers = numbers.into_iter();
    let mut num = numbers.next().unwrap();
    num.reduce();
    for other in numbers {
        num.add(other);
        num.reduce();
    }
    num
}

fn tokenize_line(input: &str) -> Number {
    use Token::*;

    let mut tokens = Vec::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '[' => {
                tokens.push(Open);
            }
            ']' => {
                tokens.push(Close);
            }
            '0'..='9' => {
                let mut num = c.to_string();
                while chars.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                    num.extend(chars.next());
                }
                tokens.push(Digit(num.parse().unwrap()))
            }
            ',' => {}
            _ => {
                panic!("Invalid char {}", c)
            }
        }
    }
    Number(tokens)
}
#[derive(PartialEq, Eq, Clone)]
struct Number(Vec<Token>);

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (token, next) in self.0.iter().zip(&self.0[1..]) {
            write!(f, "{:?}", token)?;
            if let (Token::Digit(_), Token::Digit(_)) = (token, next) {
                write!(f, ",")?
            }
        }
        write!(f, "{:?}", self.0.last().unwrap())
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Token {
    Open,
    Close,
    Digit(i32),
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "["),
            Self::Close => write!(f, "]"),
            Self::Digit(val) => write!(f, "{}", val),
        }
    }
}

impl Number {
    fn reduce(&mut self) {
        let mut reduced = false;
        while !reduced {
            reduced = true;
            if let Some(ix) = self.quad_nested() {
                reduced = false;
                self.explode(ix);
                continue;
            }
            if let Some(ix) = self.gt_10() {
                reduced = false;
                self.split(ix);
            }
        }
        debug!("reduced: {:?}", self);
    }
    fn add(&mut self, other: Self) {
        use Token::*;
        debug!("Adding {:?} to {:?}", self, other);
        self.0.insert(0, Open);
        self.0.extend(other.0);
        self.0.push(Close);
    }

    fn window(&self, ix: usize) -> Number {
        Number(self.0[ix.saturating_sub(2)..=(ix.saturating_add(2).min(self.0.len() - 1))].to_vec())
    }

    fn quad_nested(&self) -> Option<usize> {
        use Token::*;

        let mut nesting = 0;
        for (ix, c) in self.0.iter().enumerate() {
            match c {
                Open => {
                    nesting += 1;
                }
                Close => {
                    nesting -= 1;
                }
                Digit(_) => {
                    if nesting >= 5 {
                        return Some(ix);
                    }
                }
            }
        }
        None
    }
    fn explode(&mut self, ix: usize) {
        debug!("Exploding at {}: {:?} ({:?})", ix, self.window(ix), self);
        let left = self.assert_number(ix);
        let right = self.assert_number(ix + 1);
        if let Some(left_ix) = self.next_number((0..ix).rev()) {
            self.inc_at(left_ix, left)
        }
        if let Some(right_ix) = self.next_number(ix + 2..self.0.len()) {
            self.inc_at(right_ix, right)
        }
        self.assert_open(ix - 1);
        self.assert_close(ix + 2);
        drop(self.0.drain((ix - 1)..=(ix + 2)));
        self.0.insert(ix - 1, Token::Digit(0));
        debug!(
            "{}-> {:?}",
            " ".repeat("Exploding at".len() + ix.to_string().len() + 8),
            self
        );
    }

    fn gt_10(&self) -> Option<usize> {
        for (ix, tok) in self.0.iter().enumerate() {
            match tok {
                Token::Digit(val) if *val >= 10 => return Some(ix),
                _ => {}
            }
        }
        None
    }
    fn split(&mut self, ix: usize) {
        use Token::*;

        debug!("Splitting at {}: {:?}", ix, self);
        let num = self.assert_number(ix);
        let left = (num as f32 / 2.0).floor() as i32;
        let right = (num as f32 / 2.0).ceil() as i32;
        self.0[ix] = Open;
        self.0.insert(ix + 1, Digit(left));
        self.0.insert(ix + 2, Digit(right));
        self.0.insert(ix + 3, Close);
        debug!(
            "{}-> {:?}",
            " ".repeat("Splitting at".len() + ix.to_string().len()),
            self
        );
    }

    fn inc_at(&mut self, ix: usize, inc_by: i32) {
        let new_number = self.assert_number(ix) + inc_by;
        self.0[ix] = Token::Digit(new_number);
    }

    fn assert_number(&self, ix: usize) -> i32 {
        match self.0[ix] {
            Token::Digit(val) => val,
            tok => {
                panic!("Expected number at {}, but was {:?}, {:?}", ix, tok, self)
            }
        }
    }
    fn assert_open(&self, ix: usize) {
        match self.0[ix] {
            Token::Open => {}
            tok => {
                panic!("Expected open bracket at {}, but was {:?}", ix, tok)
            }
        }
    }
    fn assert_close(&self, ix: usize) {
        match self.0[ix] {
            Token::Close => {}
            tok => {
                panic!("Expected close bracket at {}, but was {:?}", ix, tok)
            }
        }
    }

    fn next_number(&self, ixes: impl Iterator<Item = usize>) -> Option<usize> {
        for ix in ixes {
            if let Token::Digit(_) = self.0[ix] {
                return Some(ix);
            }
        }
        None
    }
}

fn parse(input: &str) -> Vec<Number> {
    input.lines().map(tokenize_line).collect_vec()
}

enum NumberTree {
    Pair(Box<(NumberTree, NumberTree)>),
    Digit(i32),
}

impl NumberTree {
    fn from(number: &Number) -> Self {
        let (tree, tokens) = Self::parse(&number.0);
        assert!(tokens.is_empty(), "Expected to be empty: {:?}", tokens);
        tree
    }
    fn parse(tokens: &[Token]) -> (Self, &[Token]) {
        match tokens.first() {
            Some(token) => match token {
                Token::Open => {
                    let (first, tokens) = Self::parse(&tokens[1..]);
                    let (second, tokens) = Self::parse(tokens);
                    assert_eq!(tokens.first(), Some(&Token::Close));
                    (NumberTree::Pair(Box::new((first, second))), &tokens[1..])
                }
                Token::Close => panic!(
                    "I think I should consume those in the Open block ({:?})",
                    Number(tokens.to_owned())
                ),
                Token::Digit(digit) => (NumberTree::Digit(*digit), &tokens[1..]),
            },
            None => panic!("End"),
        }
    }
    fn magnitude(&self) -> i32 {
        match self {
            NumberTree::Pair(p) => 3 * p.0.magnitude() + 2 * p.1.magnitude(),
            NumberTree::Digit(digit) => *digit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_name() {
    //     dbg!(&parse("[[[[4,3],4],4],[7,[[8,4],9]]]")[0]);
    //     let mut node = parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
    //         .into_iter()
    //         .next()
    //         .unwrap();
    //     assert!(node.quad_nested().is_some());
    // }

    #[test]
    fn test_tokenize() {
        use Token::*;
        assert_eq!(
            tokenize_line("[1,[2,13]]"),
            Number(vec![
                Open,
                Digit(1),
                Open,
                Digit(2),
                Digit(13),
                Close,
                Close
            ])
        )
    }

    fn assert_explodes_into(input: &str, expected: &str) {
        let mut num = tokenize_line(input);
        let ix = num.quad_nested().unwrap();
        debug!("Exploding at {}: {:?}", ix, num);
        num.explode(ix);
        debug!("-> {:?}", num);
        assert_eq!(num, tokenize_line(expected));
    }

    fn assert_splits_into(input: &str, expected: &str) {
        let mut num = tokenize_line(input);
        let ix = num.gt_10().unwrap();
        debug!("Splitting at {}: {:?}", ix, num);
        num.split(ix);
        debug!("-> {:?}", num);
        assert_eq!(num, tokenize_line(expected));
    }

    #[test]
    fn test_explode() {
        assert_explodes_into("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        assert_explodes_into("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        assert_explodes_into(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        assert_explodes_into(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    fn text_split() {
        assert_splits_into("[1,[10,3]]", "[1,[[5,5],3]]");
        assert_splits_into("[1,[11,3]]", "[1,[[5,6],3]]");
    }

    #[test]
    fn test_tree_parsing() {
        NumberTree::from(&tokenize_line("[1,[2,3]]"));
        NumberTree::from(&tokenize_line(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
        ));
    }
    #[test]
    fn example() {
        let mut num = tokenize_line("[[[[4,3],4],4],[7,[[8,4],9]]]");
        num.add(tokenize_line("[1,1]"));
        num.reduce();
        assert_eq!(num, tokenize_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

        assert_eq!(
            sum(parse(
                "[1,1]
[2,2]
[3,3]
[4,4]"
            )),
            tokenize_line("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        );

        assert_eq!(
            sum(parse(
                "[1,1]
[2,2]
[3,3]
[4,4]"
            )),
            tokenize_line("[[[[1,1],[2,2]],[3,3]],[4,4]]")
        );

        assert_eq!(
            sum(parse(
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"
            )),
            tokenize_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );

        let numbers = parse(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        );
        assert_eq!(part1(numbers), 4140);
    }
}
