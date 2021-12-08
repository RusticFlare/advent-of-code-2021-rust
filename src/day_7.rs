use itertools::Itertools;
use std::cmp;

pub mod input;

pub fn part_1(input: &str) -> i32 {
    let crabs: Vec<i32> = input
        .split(",")
        .map(|n| n.parse().unwrap())
        .sorted()
        .collect();
    let target = crabs[crabs.len() / 2];
    return crabs.iter().map(|&crab| (crab - target).abs()).sum();
}

pub fn part_2(input: &str) -> i32 {
    let crabs: Vec<i32> = input.split(",").map(|n| n.parse().unwrap()).collect();
    let target: i32 = AAAA;
    return crabs
        .iter()
        .map(|&crab| (crab - target).abs())
        .map(|d| d * (d + 1) / 2)
        .sum();
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(37, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(168, part_2(input::TEST_INPUT));
    }
}
