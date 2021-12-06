use itertools::Itertools;
pub mod input;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<u16>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<u16>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

#[cfg(test)]
mod test {

    use super::part_1;
    use super::part_2;
    mod input;

    #[test]
    fn test_part_1() {
        assert_eq!(7, part_1(input::TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(5, part_2(input::TEST_INPUT));
    }
}
