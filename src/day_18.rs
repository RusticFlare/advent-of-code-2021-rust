use regex::Regex;
use std::ops::Add;

pub mod input;

#[derive(PartialEq, Eq, Clone)]
enum SnailfishNumber {
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
    Value(u32),
}

impl SnailfishNumber {
    fn explode(&self, depth: i32) -> (u32, Self, u32) {
        match self {
            SnailfishNumber::Pair(l, r) => match (l.as_ref(), r.as_ref()) {
                (SnailfishNumber::Value(left), SnailfishNumber::Value(right)) => {
                    if depth >= 4 {
                        (*left, SnailfishNumber::Value(0), *right)
                    } else {
                        (0, self.clone(), 0)
                    }
                }
                (_, _) => {
                    let (le, ex, ri) = l.explode(depth + 1);
                    if ex != *l.as_ref() {
                        (
                            le,
                            SnailfishNumber::Pair(Box::new(ex), Box::new(r.add_left(ri))),
                            0,
                        )
                    } else {
                        let (le, ex, ri) = r.explode(depth + 1);
                        (
                            0,
                            SnailfishNumber::Pair(Box::new(l.add_right(le)), Box::new(ex)),
                            ri,
                        )
                    }
                }
            },
            SnailfishNumber::Value(_) => (0, self.clone(), 0),
        }
    }

    fn add_left(&self, value: u32) -> Self {
        match self {
            SnailfishNumber::Pair(l, r) => {
                SnailfishNumber::Pair(Box::new(l.add_left(value)), r.clone())
            }
            SnailfishNumber::Value(v) => SnailfishNumber::Value(v + value),
        }
    }

    fn add_right(&self, value: u32) -> Self {
        match self {
            SnailfishNumber::Pair(l, r) => {
                SnailfishNumber::Pair(l.clone(), Box::new(r.add_right(value)))
            }
            SnailfishNumber::Value(v) => SnailfishNumber::Value(v + value),
        }
    }

    fn split(&self) -> Self {
        match self {
            SnailfishNumber::Pair(l, r) => {
                if l.split() != *l.as_ref() {
                    SnailfishNumber::Pair(Box::new(l.split()), r.clone())
                } else {
                    SnailfishNumber::Pair(l.clone(), Box::new(r.split()))
                }
            }
            SnailfishNumber::Value(v) => {
                if *v >= 10 {
                    SnailfishNumber::Pair(
                        Box::new(SnailfishNumber::Value(v / 2)),
                        Box::new(SnailfishNumber::Value(if v % 2 == 0 {
                            v / 2
                        } else {
                            v / 2 + 1
                        })),
                    )
                } else {
                    self.clone()
                }
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailfishNumber::Pair(l, r) => l.magnitude() * 3 + r.magnitude() * 2,
            SnailfishNumber::Value(v) => *v,
        }
    }

    fn reduce(&self) -> Self {
        let mut old = self.clone();
        loop {
            let (_, mut new, _) = old.explode(0);
            if new == old {
                new = new.split();
            };
            if new == old {
                break;
            };
            old = new;
        }
        old
    }

    fn print(&self) -> String {
        match self {
            SnailfishNumber::Pair(l, r) => format!("[{},{}]", l.print(), r.print()),
            SnailfishNumber::Value(v) => format!("{}", v),
        }
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        SnailfishNumber::Pair(Box::new(self), Box::new(rhs))
    }
}

fn snailfish_number(input: &str) -> SnailfishNumber {
    if input.starts_with("[") {
        let captures = Regex::new(r"\[(?P<left>([^\[\]]*|\[([^\[\]]*|\[([^\[\]]*|\[([^\[\]]*|\[[^\[\]]*])*])*])*])*|\d),(?P<right>([^\[\]]*|\[([^\[\]]*|\[([^\[\]]*|\[([^\[\]]*|\[[^\[\]]*])*])*])*])*|\d)]")
            .unwrap()
            .captures(input)
            .unwrap();
        let left = snailfish_number(captures.name("left").unwrap().as_str());
        let right = snailfish_number(captures.name("right").unwrap().as_str());
        SnailfishNumber::Pair(Box::new(left), Box::new(right))
    } else {
        SnailfishNumber::Value(input.parse().unwrap())
    }
}

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| snailfish_number(l).reduce())
        .reduce(|acc, item| (acc + item).reduce())
        .unwrap()
        .magnitude()
}

pub fn part_2(input: &str) -> u32 {
    let snailfish_numbers: Vec<SnailfishNumber> = input
        .lines()
        .map(|l| snailfish_number(l).reduce())
        .collect();
    snailfish_numbers
        .iter()
        .map(|l| {
            snailfish_numbers
                .iter()
                .map(|r| (l.clone() + r.clone()).reduce().magnitude())
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::part_1;
    use super::part_2;
    use crate::day_18::snailfish_number;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(4140, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_explode() {
        assert_eq!(
            "[[[[0,9],2],3],4]",
            snailfish_number("[[[[[9,8],1],2],3],4]")
                .explode(0)
                .1
                .print()
        );
        assert_eq!(
            "[7,[6,[5,[7,0]]]]",
            snailfish_number("[7,[6,[5,[4,[3,2]]]]]")
                .explode(0)
                .1
                .print()
        );
        assert_eq!(
            "[[6,[5,[7,0]]],3]",
            snailfish_number("[[6,[5,[4,[3,2]]]],1]")
                .explode(0)
                .1
                .print()
        );
        assert_eq!(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            snailfish_number("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
                .explode(0)
                .1
                .print()
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3993, part_2(input::TEST_INPUT));
    }
}
