use itertools::Itertools;
use regex::Regex;
use std::u32;

pub mod input;

pub fn part_1(input: &str) -> i32 {
    let y_min: i32 = Regex::new("y=(?P<y_min>.*)\\.\\.")
        .unwrap()
        .captures(input)
        .unwrap()
        .name("y_min")
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    (1..(y_min.abs())).sum()
}

pub fn part_2(input: &str) -> usize {
    let captures =
        Regex::new("x=(?P<x_min>.*)\\.\\.(?P<x_max>.*), y=(?P<y_min>.*)\\.\\.(?P<y_max>.*)")
            .unwrap()
            .captures(input)
            .unwrap();
    let x_min: i32 = captures.name("x_min").unwrap().as_str().parse().unwrap();
    let x_max: i32 = captures.name("x_max").unwrap().as_str().parse().unwrap();
    let y_min: i32 = captures.name("y_min").unwrap().as_str().parse().unwrap();
    let y_max: i32 = captures.name("y_max").unwrap().as_str().parse().unwrap();

    let x_target_range = x_min..=x_max;
    let y_target_range = y_min..=y_max;

    let x_ans_min = (1..x_min)
        .filter(|&x| x_target_range.contains(&triangle(x)))
        .next()
        .unwrap();
    let x_ans_max = x_max;

    let y_ans_min = y_min;
    let y_ams_max = y_min.abs() - 1;

    let is_valid = |mut x, mut y| -> bool {
        let mut x_pos = 0;
        let mut y_pos = 0;
        while x_pos <= x_max && y_pos >= y_min {
            if x_target_range.contains(&x_pos) && y_target_range.contains(&y_pos) {
                return true;
            }
            x_pos = x_pos + x;
            y_pos = y_pos + y;
            x = if x > 0 {
                x - 1
            } else if x < 0 {
                x + 1
            } else {
                0
            };
            y = y - 1;
        }
        return false;
    };

    (x_ans_min..=x_ans_max)
        .map(|x| (y_ans_min..=y_ams_max).filter(|&y| is_valid(x, y)).count())
        .sum()
}

fn triangle(x: i32) -> i32 {
    x * (x + 1) / 2
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(45, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(112, part_2(input::TEST_INPUT));
    }
}
