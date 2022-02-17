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

pub fn part_2(input: &str) -> u64 {
    0
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
        assert_eq!(0, part_2(input::TEST_INPUT));
    }
}
