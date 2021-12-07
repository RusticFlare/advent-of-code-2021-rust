use regex::Regex;
use std::collections::HashMap;

pub mod input;

struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    fn is_vertical(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_horizontal(&self) -> bool {
        self.x1 == self.x2
    }

    fn points(&self) -> Vec<(u32, u32)> {
        if self.is_vertical() {
            if self.x1 < self.x2 {
                self.x1..=self.x2
            } else {
                self.x2..=self.x1
            }
            .map(|x| (x, self.y1))
            .collect()
        } else if self.is_horizontal() {
            if self.y1 < self.y2 {
                self.y1..=self.y2
            } else {
                self.y2..=self.y1
            }
            .map(|y| (self.x1, y))
            .collect()
        } else {
            let xs: Vec<u32> = if self.x1 <= self.x2 {
                (self.x1..=self.x2).collect()
            } else {
                (self.x2..=self.x1).rev().collect()
            };
            let ys: Vec<u32> = if self.y1 <= self.y2 {
                (self.y1..=self.y2).collect()
            } else {
                (self.y2..=self.y1).rev().collect()
            };
            xs.iter().zip(ys.iter()).map(|(&x,&y)| (x,y)).collect()
        }
    }
}

pub fn part_1(input: &str) -> usize {
    parse_lines(input)
        .iter()
        .filter(|line| line.is_horizontal_or_vertical())
        .flat_map(|line| line.points())
        .fold(HashMap::new(), |mut acc, point| {
            *acc.entry(point).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter(|(_, &count)| count > 1)
        .count()
}

pub fn part_2(input: &str) -> u32 {
    return 0;
}

fn parse_lines(input: &str) -> Vec<Line> {
    let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();

    return input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|caps| Line {
            x1: caps.name("x1").unwrap().as_str().parse().unwrap(),
            y1: caps.name("y1").unwrap().as_str().parse().unwrap(),
            x2: caps.name("x2").unwrap().as_str().parse().unwrap(),
            y2: caps.name("y2").unwrap().as_str().parse().unwrap(),
        })
        .collect();
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(5, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(12, part_2(input::TEST_INPUT));
    }
}
